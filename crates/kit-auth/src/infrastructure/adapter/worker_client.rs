use kit_errors::errors::Errors;
use kit_worker::jobs::email::{EmailTemplate, SendEmailJob};
use kit_worker::nats::streams::EMAIL_SUBJECT;
use serde::Serialize;
use tracing::info;

/// Worker client for sending jobs to background workers
#[derive(Clone)]
pub struct WorkerClient {
    // Internal client would be here
}

impl WorkerClient {
    /// Create a new worker client
    pub fn new() -> Self {
        Self {}
    }

    /// Publish a job to NATS JetStream
    pub async fn publish(&self, _subject: String, _payload: bytes::Bytes) -> Result<(), Errors> {
        // Implementation would publish to NATS
        Ok(())
    }
}

impl Default for WorkerClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Publish a job to NATS JetStream
pub async fn publish_job<T: Serialize>(
    worker: &WorkerClient,
    subject: &str,
    job: &T,
) -> Result<(), Errors> {
    let payload = serde_json::to_vec(job).map_err(|_| Errors::WorkerServiceConnectionFailed)?;

    worker
        .publish(subject.to_string(), payload.into())
        .await
        .map_err(|_| Errors::WorkerServiceConnectionFailed)?;

    Ok(())
}

/// Push a verification email job to the worker queue
pub async fn send_verification_email(
    worker: &WorkerClient,
    email_to: &str,
    username: &str,
    verification_token: &str,
    valid_minutes: u64,
) -> Result<(), Errors> {
    info!("Queuing verification email job for {}", email_to);

    let job = SendEmailJob {
        to: email_to.to_string(),
        subject: "Verify your email".to_string(),
        template: EmailTemplate::Verification {
            username: username.to_string(),
            token: verification_token.to_string(),
            valid_minutes,
        },
    };

    publish_job(worker, EMAIL_SUBJECT, &job).await?;

    info!("Verification email job queued for {}", email_to);
    Ok(())
}

/// Push a password reset email job to the worker queue
pub async fn send_password_reset_email(
    worker: &WorkerClient,
    email_to: &str,
    handle: &str,
    reset_token: &str,
    valid_minutes: u64,
) -> Result<(), Errors> {
    info!("Queuing password reset email job for {}", email_to);

    let job = SendEmailJob {
        to: email_to.to_string(),
        subject: "Reset your password".to_string(),
        template: EmailTemplate::PasswordReset {
            handle: handle.to_string(),
            token: reset_token.to_string(),
            valid_minutes,
        },
    };

    publish_job(worker, EMAIL_SUBJECT, &job).await?;

    info!("Password reset email job queued for {}", email_to);
    Ok(())
}

/// Push an email change verification job to the worker queue
pub async fn send_email_change_verification(
    worker: &WorkerClient,
    new_email: &str,
    username: &str,
    token: &str,
    valid_minutes: u64,
) -> Result<(), Errors> {
    info!("Queuing email change verification job for {}", new_email);

    let job = SendEmailJob {
        to: new_email.to_string(),
        subject: "Confirm your email change".to_string(),
        template: EmailTemplate::EmailChange {
            username: username.to_string(),
            token: token.to_string(),
            valid_minutes,
        },
    };

    publish_job(worker, EMAIL_SUBJECT, &job).await?;

    info!("Email change verification job queued for {}", new_email);
    Ok(())
}
