# Issue 496 Emergency Cancel Scaffold

Tracks implementation for admin-only emergency job cancellation.

Planned scope:
- Add `admin_emergency_cancel(admin, job_id, return_to_client_bps, emergency_cancel_reason)`.
- Support active stuck states: Open, InProgress, SubmittedForReview, and Disputed.
- Split escrowed funds by basis points between client and freelancer.
- Emit a transparent emergency cancellation event with reason metadata.
- Add frontend admin form, unit tests, and emergency operations documentation.
