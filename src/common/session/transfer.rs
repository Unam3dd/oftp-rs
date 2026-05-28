//! Transfert de fichier OFTP (SFID → DATA → EFID) côté émetteur et récepteur.

use std::path::{Path, PathBuf};

use crate::commands::{
    sfid::{Sfid, SfidFormat},
    Cd, Cdt, Data, Eerp, Efid, Efpa, Esid, OftpExchangeBuffer, Rtr, Sfpa,
};
use crate::state::State;
use thiserror::Error;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::time::{timeout, Duration};
use tracing::{debug, info, warn};

use super::OftpSession;

const STH_SIZE: u32 = 4;

#[derive(Debug, Error)]
pub enum TransferError {
    #[error("fichier introuvable: {0}")]
    FileNotFound(String),

    #[error("lecture fichier: {0}")]
    Read(std::io::Error),

    #[error("écriture fichier: {0}")]
    Write(std::io::Error),

    #[error("SFID: {0}")]
    Sfid(#[from] crate::commands::SfidFieldError),

    #[error("démarrage fichier refusé (SFNA raison {reason})")]
    StartRejected { reason: u8 },

    #[error("fin de fichier refusée (EFNA raison {reason})")]
    EndRejected { reason: u8 },

    #[error("PDU inattendu pendant le transfert")]
    UnexpectedPdu,

    #[error("SSID pair non disponible (handshake requis)")]
    MissingPeerSsid,

    #[error("fichier vide: {0}")]
    EmptyFile(String),
}

impl OftpSession {
    /// Construit un SFID à partir d'un fichier local (format non structuré).
    pub fn sfid_from_path(path: &Path) -> Result<Sfid, TransferError> {
        let meta = std::fs::metadata(path).map_err(|_| {
            TransferError::FileNotFound(path.display().to_string())
        })?;
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| TransferError::FileNotFound(path.display().to_string()))?;

        let bytes = meta.len();
        // mendelson / RFC : SFIDDSN en majuscules (jeu AN ODETTE).
        let dsn = name.to_ascii_uppercase();
        let mut sfid = Sfid::default();
        sfid.format = SfidFormat::Unstructured;
        sfid.set_dsn(&dsn)?;
        sfid.file_size_k = bytes.div_ceil(1024).max(1);
        sfid.orig_file_size_k = sfid.file_size_k;
        sfid.restart_pos = 0;
        Ok(sfid)
    }

    /// Taille max du payload DATA (octets), selon SSID négocié.
    pub fn negotiated_max_data_payload(&self) -> Result<usize, TransferError> {
        let _ = self.peer_ssid.as_ref().ok_or(TransferError::MissingPeerSsid)?;
        let stb = self.negotiated_exchange_buffer_size();
        let oeb_max = stb.saturating_sub(STH_SIZE) as usize;
        Ok(oeb_max.saturating_sub(1))
    }

    pub fn negotiated_exchange_buffer_size(&self) -> u32 {
        let local = self.options.local_ssid.buffer_size;
        let peer = self
            .peer_ssid
            .as_ref()
            .map(|s| s.buffer_size)
            .unwrap_or(local);
        local.min(peer)
    }

    pub fn negotiated_credit(&self) -> u16 {
        let local = self.options.local_ssid.credit;
        let peer = self.peer_ssid.as_ref().map(|s| s.credit).unwrap_or(local);
        local.min(peer).max(1)
    }

    /// Envoie un fichier (rôle Speaker : SFID → DATA* → EFID).
    pub async fn run_send_file(&mut self, path: &Path) -> Result<u64, super::error::SessionError> {
        let max_payload = self.negotiated_max_data_payload()?;
        let credit_window = self.negotiated_credit();

        let mut sfid = Self::sfid_from_path(path)?;
        if let Some(peer) = &self.peer_ssid {
            sfid.set_dest(trim_ssid_field(&peer.code))
                .map_err(TransferError::Sfid)?;
        }
        sfid.set_orig(trim_ssid_field(&self.options.local_ssid.code))
            .map_err(TransferError::Sfid)?;

        info!(
            file = %path.display(),
            dsn = %trim_bytes(&sfid.dsn),
            file_size_k = sfid.file_size_k,
            max_payload,
            credit_window,
            "démarrage envoi fichier"
        );
        self.write_pdu(&OftpExchangeBuffer::Sfid(sfid.clone())).await?;
        self.state = State::Opop;

        let (_sth, answer) = self.read_pdu().await?;
        match answer {
            OftpExchangeBuffer::Sfpa(_) => {
                debug!("SFPA reçu");
                self.state = State::Opo;
            }
            OftpExchangeBuffer::Sfna(sfna) => {
                return Err(TransferError::StartRejected {
                    reason: sfna.reason,
                }
                .into());
            }
            _ => return Err(TransferError::UnexpectedPdu.into()),
        }

        let file_bytes = std::fs::read(path).map_err(|e| TransferError::Read(e))?;
        if file_bytes.is_empty() {
            return Err(TransferError::EmptyFile(path.display().to_string()).into());
        }

        let mut offset = 0usize;
        let mut total_bytes = 0u64;
        let mut credit_left = credit_window;

        while offset < file_bytes.len() {
            if credit_left == 0 {
                let (_sth, pdu) = self.read_pdu().await?;
                match pdu {
                    OftpExchangeBuffer::Cdt(_) => {
                        credit_left = credit_window;
                        debug!("CDT reçu — crédit renouvelé");
                    }
                    _ => return Err(TransferError::UnexpectedPdu.into()),
                }
            }

            let end = (offset + max_payload).min(file_bytes.len());
            let chunk = &file_bytes[offset..end];
            self.write_pdu(&OftpExchangeBuffer::Data(Data {
                payload: chunk.to_vec(),
            }))
            .await?;
            total_bytes += chunk.len() as u64;
            offset = end;
            credit_left -= 1;
            debug!(
                chunk_len = chunk.len(),
                offset,
                total = total_bytes,
                "DATA envoyé"
            );
        }

        info!(total_bytes, "tous les blocs DATA envoyés");

        let efid = OftpExchangeBuffer::Efid(Efid {
            record_count: 0,
            unit_count: total_bytes,
        });
        self.write_pdu(&efid).await?;
        self.state = State::Clop;

        let (_sth, end_answer) = self.read_pdu().await?;
        match end_answer {
            OftpExchangeBuffer::Efpa(efpa) => {
                debug!(
                    change_direction = efpa.change_direction,
                    "EFPA reçu"
                );
                if efpa.change_direction {
                    debug!("EFPA demande CD — envoi CD");
                    self.write_pdu(&OftpExchangeBuffer::Cd(Cd)).await?;
                    info!(
                        "EFPA(CD=Y) : mendelson enverra souvent l'EERP sur une \
                         connexion de rappel — lancer oftp-server sur le port \
                         partenaire (ex. 3306), pas 3305"
                    );
                }
                self.wait_for_eerp().await?;
                self.write_pdu(&OftpExchangeBuffer::Rtr(Rtr)).await?;
                debug!("RTR envoyé — confirmation bout-en-bout OK");
                self.state = State::IdleSp;
            }
            OftpExchangeBuffer::Efna(efna) => {
                return Err(TransferError::EndRejected {
                    reason: efna.reason,
                }
                .into());
            }
            _ => return Err(TransferError::UnexpectedPdu.into()),
        }

        info!(total_bytes, "fichier envoyé");
        Ok(total_bytes)
    }

    /// Après handshake répondeur : réception fichier **ou** rappel EERP (mendelson).
    pub async fn run_responder_service(
        &mut self,
        output_dir: &Path,
    ) -> Result<(), super::error::SessionError> {
        let first = match timeout(Duration::from_secs(8), self.read_pdu()).await {
            Ok(Ok(pdu)) => pdu,
            Ok(Err(err)) => return Err(err.into()),
            Err(_) => {
                info!("aucun PDU après handshake (test connexion mendelson ?)");
                return Ok(());
            }
        };
        let (_sth, first) = first;
        match first {
            OftpExchangeBuffer::Sfid(sfid) => {
                self.receive_from_sfid(sfid, output_dir).await?;
            }
            OftpExchangeBuffer::Eerp(_) => {
                info!(
                    "EERP entrant (rappel mendelson après envoi vers lui) — \
                     le serveur répond RTR et n'envoie pas d'EERP sur cette connexion"
                );
                self.write_pdu(&OftpExchangeBuffer::Rtr(Rtr)).await?;
                info!("RTR envoyé — accusé EERP mendelson pris en compte");
            }
            other => {
                warn!(?other, "premier PDU inattendu côté répondeur");
                return Err(TransferError::UnexpectedPdu.into());
            }
        }
        Ok(())
    }

    /// Reçoit un fichier (rôle Listener : SFID → DATA* → EFID).
    pub async fn run_receive_file(
        &mut self,
        output_dir: &Path,
    ) -> Result<PathBuf, super::error::SessionError> {
        let (_sth, start) = self.read_pdu().await?;
        let sfid = match start {
            OftpExchangeBuffer::Sfid(sfid) => sfid,
            _ => return Err(TransferError::UnexpectedPdu.into()),
        };
        self.receive_from_sfid(sfid, output_dir).await
    }

    async fn receive_from_sfid(
        &mut self,
        sfid: Sfid,
        output_dir: &Path,
    ) -> Result<PathBuf, super::error::SessionError> {
        let _ = self.peer_ssid.as_ref().ok_or(TransferError::MissingPeerSsid)?;
        let credit_window = self.negotiated_credit();

        let dsn = trim_bytes(&sfid.dsn);
        let out_path = output_dir.join(dsn);
        info!(path = %out_path.display(), "réception fichier");

        self.write_pdu(&OftpExchangeBuffer::Sfpa(Sfpa::default()))
            .await?;
        self.state = State::Opi;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&out_path)
            .await
            .map_err(|e| TransferError::Write(e))?;

        let mut credit_left = credit_window;
        let mut total_bytes = 0u64;

        loop {
            let (_sth, pdu) = self.read_pdu().await?;
            match pdu {
                OftpExchangeBuffer::Data(data) => {
                    file.write_all(&data.payload)
                        .await
                        .map_err(|e| TransferError::Write(e))?;
                    total_bytes += data.payload.len() as u64;
                    credit_left -= 1;
                    if credit_left == 0 {
                        self.write_pdu(&OftpExchangeBuffer::Cdt(Cdt)).await?;
                        credit_left = credit_window;
                        debug!("CDT envoyé");
                    }
                }
                OftpExchangeBuffer::Efid(efid) => {
                    file.flush().await.map_err(|e| TransferError::Write(e))?;
                    if total_bytes > efid.unit_count {
                        file.set_len(efid.unit_count)
                            .await
                            .map_err(|e| TransferError::Write(e))?;
                        warn!(
                            received = total_bytes,
                            expected = efid.unit_count,
                            "fichier tronqué au comptage EFID (padding DATA retiré)"
                        );
                        total_bytes = efid.unit_count;
                    }
                    debug!(
                        expected_units = efid.unit_count,
                        received = total_bytes,
                        "EFID reçu"
                    );

                    // Même séquence que mendelson côté récepteur : EFPA(CD=Y) → CD → EERP.
                    self.write_pdu(&OftpExchangeBuffer::Efpa(Efpa {
                        change_direction: true,
                    }))
                    .await?;
                    self.wait_for_cd_after_efpa().await?;

                    let eerp = Eerp::from_sfid(&sfid);
                    self.write_pdu(&OftpExchangeBuffer::Eerp(eerp)).await?;
                    info!(
                        dsn = %trim_bytes(&sfid.dsn),
                        bytes = total_bytes,
                        "EERP envoyé au pair (accusé de réception bout-en-bout)"
                    );

                    match timeout(Duration::from_secs(60), self.wait_for_rtr()).await {
                        Ok(Ok(())) => debug!("RTR reçu — émetteur prêt"),
                        Ok(Err(err)) => return Err(err),
                        Err(_) => warn!(
                            "pas de RTR dans les 60 s — l'EERP a déjà été envoyé sur cette session"
                        ),
                    }
                    self.state = State::IdleLi;
                    break;
                }
                other => {
                    warn!(?other, "PDU inattendu en réception");
                    return Err(TransferError::UnexpectedPdu.into());
                }
            }
        }

        file.flush().await.map_err(|e| TransferError::Write(e))?;
        info!(path = %out_path.display(), total_bytes, "fichier reçu");
        Ok(out_path)
    }

    /// Après EFPA(CD=Y) en réception : l'émetteur (Speaker) doit envoyer CD avant l'EERP.
    async fn wait_for_cd_after_efpa(&mut self) -> Result<(), super::error::SessionError> {
        loop {
            let (_sth, pdu) = self.read_pdu().await?;
            match pdu {
                OftpExchangeBuffer::Cd(_) => {
                    debug!("CD reçu de l'émetteur — envoi EERP");
                    return Ok(());
                }
                other => {
                    warn!(?other, "PDU inattendu en attente de CD après EFPA");
                    return Err(TransferError::UnexpectedPdu.into());
                }
            }
        }
    }

    /// Attend un EERP du récepteur (ignore un éventuel CD préalable).
    async fn wait_for_eerp(&mut self) -> Result<(), super::error::SessionError> {
        loop {
            let (_sth, pdu) = self.read_pdu().await?;
            match pdu {
                OftpExchangeBuffer::Cd(_) => {
                    debug!(
                        "CD reçu du pair (réponse au nôtre) — toujours en attente d'EERP \
                         sur cette session ou via rappel TCP"
                    );
                }
                OftpExchangeBuffer::Eerp(_) => {
                    info!("EERP reçu sur la session courante");
                    return Ok(());
                }
                _ => return Err(TransferError::UnexpectedPdu.into()),
            }
        }
    }

    /// Attend un RTR de l'émetteur après envoi d'EERP.
    async fn wait_for_rtr(&mut self) -> Result<(), super::error::SessionError> {
        loop {
            let (_sth, pdu) = self.read_pdu().await?;
            match pdu {
                OftpExchangeBuffer::Cd(_) => {
                    debug!("CD reçu avant RTR — ignoré");
                }
                OftpExchangeBuffer::Rtr(_) => return Ok(()),
                _ => return Err(TransferError::UnexpectedPdu.into()),
            }
        }
    }

    /// Envoie ESID (fin de session normale) et ferme la TCP.
    pub async fn end_session(&mut self) -> Result<(), super::error::SessionError> {
        self.write_pdu(&OftpExchangeBuffer::Esid(Esid::normal()))
            .await?;
        self.close().await
    }
}

fn trim_ssid_field(field: &[u8; 25]) -> &str {
    let end = field
        .iter()
        .rposition(|&b| b != b' ')
        .map(|i| i + 1)
        .unwrap_or(0);
    std::str::from_utf8(&field[..end]).unwrap_or("")
}

fn trim_bytes(field: &[u8]) -> String {
    let end = field
        .iter()
        .rposition(|&b| b != b' ')
        .map(|i| i + 1)
        .unwrap_or(0);
    String::from_utf8_lossy(&field[..end]).into_owned()
}
