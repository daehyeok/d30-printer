# Workflow - d30-printer

## Development Process
- **Task-Driven Development:** All work must be broken down into discrete tasks in the track's `plan.md`.
- **Atomic Commits:** Each completed task should be committed individually with a clear, descriptive message.
- **No Test Requirement:** This project does not require automated test coverage at this time. Focus on functional verification through manual testing or CLI output verification.

## Task Workflow

All tasks follow a strict lifecycle:

1. **Select Task:** Choose the next available task from `plan.md` in sequential order.
2. **Mark In Progress:** Before beginning work, edit `plan.md` and change the task from `[ ]` to `[~]`.
3. **Implementation:** Write the code necessary to fulfill the task requirements.
4. **Verification:** Manually verify the changes by running the CLI and checking the output or printer behavior.
5. **Commit Code Changes:** Stage all changes and commit with a descriptive message.
6. **Update Plan:** Mark the task as complete `[x]` in `plan.md` and include the commit hash.

## Phase Completion Verification
- At the end of each phase, a manual verification step is required to ensure the implementation aligns with the track's specification.

## Documentation
- Keep the `README.md` and `conductor/` artifacts updated as the project evolves.

## Commit Guidelines
- Use the format: `<type>(<scope>): <description>`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `chore`.
