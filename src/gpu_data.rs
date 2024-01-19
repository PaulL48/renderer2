// This is a space to explore a generalized way to handle vertex buffer data and bind group data

// One thing that must be considered is that there are a few common ways that a GPU_data buffer
// is going to be updated: on init, once per-frame (with stale detection),
// and when and where they will be bound: per-command, per-pipeline
