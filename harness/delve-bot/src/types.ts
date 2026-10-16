export interface Post {
  uri: string;
  cid: string;
  did: string;
  handle: string;
  text: string;
  createdAt: string;
  rootUri: string;
  parentUri: string | null;
  mentions: string[];
  isBot: boolean;
}

export interface IngestEvent {
  cursor: number;
  operation: "create" | "update" | "delete";
  uri: string;
  post: Post | null;
  rejectionReason?: "invalid_jetstream_record";
}

export interface Config {
  did: string;
  handle: string;
  dryRun: boolean;
  livePosting: boolean;
  liveStreaming: boolean;
  database: string;
  killFile: string;
  jetstreamUrl: string;
  appviewUrl: string;
  pdsUrl: string;
  anthropicUrl: string;
  blockedDids: string[];
  optedOutDids: string[];
  botDids: string[];
  contentCheck: { model: "claude-opus-5-5"; timeoutSeconds: number };
  streaming: {
    enabled: boolean;
    includeLink: boolean;
    concurrent: 1;
    maxSeconds?: number | null;
    width: number;
    height: number;
    fps: number;
    videoKbps: number;
    audioKbps: number;
    ingestUrl: string;
    stationUrl: string;
    viewerUrl: string;
    chromiumPath: string;
  };
  caps: {
    paintingsPerDay: number;
    running: number;
    requesterPerDay: number;
    requesterCooldownSeconds: number;
    botCooldownSeconds: number;
    botOnlyTurns: number;
    queue: number;
    queueTtlSeconds: number;
    requestChars: number;
    requestBytes: number;
    requestAgeSeconds: number;
    globalPerHour: number;
    threadPerSixHours: number;
    pendingEvents: number;
    pendingReplies: number;
    databaseBytes: number;
    eventBytes: number;
    replyChars: number;
    replyBytes: number;
    imageBytes: number;
  };
  studio: {
    medium: "oil";
    width: number;
    height: number;
    engine: 7;
    painterModel: "claude-opus-5-5";
    thinking: "low" | "medium" | "high" | "xhigh" | "max";
    template: string;
    root: string;
    sessions: string;
    harness: string;
    maxWallSeconds?: number | null;
    maxInputTokens?: number | null;
    maxOutputTokens?: number | null;
    maxCostUsd?: number | null;
  };
}

export interface Job {
  id: string;
  post: Post;
  root: { uri: string; cid: string };
  instruction: string;
  brief: string;
  studio: Config["studio"];
  folder: string;
}

export interface Painting {
  bytes: Buffer;
  width: number;
  height: number;
  alt: string;
  stopped: string | null;
}

export interface JobRow {
  id: string;
  did: string;
  root: string;
  admitted: number;
  started: number | null;
  status: "queued" | "running" | "done" | "failed" | "canceled" | "expired";
  job: string;
}

export interface Intent {
  id: string;
  did: string;
  root: string;
  rkey: string;
  record: string;
  created: number;
  sent: number | null;
  status: "pending" | "simulated" | "verified" | "canceled";
}
