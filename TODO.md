# Optional TODOs

- [ ] Add direct messaging feature (use `endpoint::post::types::NewPostOption.direct_message_to` field)
- [ ] Change `reaction` from JSONB to separate table (use `query::reaction::Reaction` struct)
- [ ] Sync frontend nu_type validation to be the same as the UI validation? Currently can cause bugs from not manually keeping setting them as the same
