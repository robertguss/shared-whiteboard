CREATE TABLE orchestration_v2_projection_messages (
  message_id TEXT PRIMARY KEY,
  thread_id TEXT NOT NULL,
  run_id TEXT,
  role TEXT NOT NULL,
  created_at TEXT NOT NULL,
  payload_json TEXT NOT NULL
);
CREATE TABLE orchestration_v2_projection_runs (
  run_id TEXT PRIMARY KEY,
  provider_thread_id TEXT
);
CREATE TABLE orchestration_v2_projection_provider_threads (
  provider_thread_id TEXT PRIMARY KEY,
  payload_json TEXT NOT NULL
);
CREATE TABLE projection_projects (
  project_id TEXT PRIMARY KEY,
  workspace_root TEXT NOT NULL
);

INSERT INTO orchestration_v2_projection_runs VALUES
  ('run-1', 'provider-thread-1'),
  ('run-2', 'provider-thread-2');
INSERT INTO orchestration_v2_projection_provider_threads VALUES
  ('provider-thread-1', '{"nativeThreadRef":{"nativeId":"00000000-fake-session-0001"}}'),
  ('provider-thread-2', '{"nativeThreadRef":{"nativeId":"00000000-fake-session-0002"}}');
INSERT INTO orchestration_v2_projection_messages VALUES
  ('message-1', 'thread-a', 'run-1', 'user', '2026-01-01T10:00:00.000Z',
   '{"createdBy":"user","creationSource":"web","text":"  Made-up question about a pretend garden  "}'),
  ('message-2', 'thread-a', 'run-1', 'assistant', '2026-01-01T10:00:05.000Z',
   '{"createdBy":"agent","creationSource":"provider","text":"Made-up answer about the pretend garden"}'),
  ('message-3', 'thread-b', 'run-2', 'user', '2026-01-01T11:00:00.000Z',
   '{"createdBy":"agent","creationSource":"mcp","text":"Made-up instruction from another agent"}'),
  ('message-4', 'thread-c', NULL, 'user', '2026-01-01T12:00:00.000Z',
   '{"createdBy":"user","creationSource":"mobile","text":"Made-up note typed on a phone"}');
INSERT INTO projection_projects VALUES
  ('project-1', '/home/example/code/pretend-garden'),
  ('project-2', '/home/example/.t3/scratch');
