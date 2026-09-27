// node --test harness/painter/test/pace.test.ts
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import { requestTokens, retryablePerMinuteQuota, TokenPace } from "../pace.ts";

test("requests wait until the last minute's input tokens plus the next fit the budget", () => {
	const p = new TokenPace(1_000_000);
	assert.equal(p.waitMs(400_000, 0), 0);
	p.record(400_000, 0);
	p.record(400_000, 10_000);
	assert.equal(p.waitMs(150_000, 20_000), 0);
	// 800K in the window + 400K: the first request has to leave the window (at 60 s)
	assert.equal(p.waitMs(400_000, 20_000), 40_000);
	assert.equal(p.waitMs(400_000, 60_001), 0);
	// a request bigger than the budget waits for an empty window, then goes
	p.record(400_000, 60_001);
	assert.equal(p.waitMs(2_000_000, 61_000), 59_001);
});

test("a request's input tokens are new plus cached", () => {
	assert.equal(requestTokens({ input: 5_808, cacheRead: 211_886 }), 217_694);
	assert.equal(requestTokens(undefined), 0);
});

// pi's error text for round 18g GEM's 429 (JSON escaped in JSON), as in p1_s1_err.txt
const GOOGLE_429 = String.raw`{"error":{"message":"{\n  \"error\": {\n    \"code\": 429,\n    \"message\": \"You exceeded your current quota, please check your plan and billing details. \\n* Quota exceeded for metric: generativelanguage.googleapis.com/generate_content_paid_tier_input_token_count, limit: 2000000, model: gemini-3.8-flash\\nPlease retry in 53.812706879s.\",\n    \"status\": \"RESOURCE_EXHAUSTED\",\n    \"details\": [{\"@type\": \"type.googleapis.com/google.rpc.QuotaFailure\", \"violations\": [{\"quotaId\": \"GenerateContentPaidTierInputTokensPerModelPerMinute\", \"quotaValue\": \"2000000\"}]}]\n  }\n}\n","code":429,"status":"Too Many Requests"}}`;

test("a per-minute quota 429 becomes a rate limit pi's retry takes", () => {
	const r = retryablePerMinuteQuota(GOOGLE_429);
	assert.ok(r);
	assert.match(r, /429 rate limit/);
	assert.match(r, /GenerateContentPaidTierInputTokensPerModelPerMinute/);
	assert.match(r, /53\.812706879s/);
	// pi-ai's non-retryable pattern must not match the rewrite
	assert.doesNotMatch(r, /quota exceeded|billing|insufficient_quota|out of budget|usage limit|available balance/i);
});

test("daily quotas, billing errors and other errors are left alone", () => {
	assert.equal(retryablePerMinuteQuota(GOOGLE_429.replace("PerModelPerMinute", "PerModelPerDay")), undefined);
	assert.equal(retryablePerMinuteQuota('429 {"error": {"code": "insufficient_quota", "message": "check your billing"}}'), undefined);
	assert.equal(retryablePerMinuteQuota("socket hang up"), undefined);
	assert.equal(retryablePerMinuteQuota(undefined), undefined);
});

test("the real error text from the round 18g session file is recognized", () => {
	const f = `${process.env.HOME}/tmp/gallery-fcf9c110/r18g/run/GEM/p1_s1_err.txt`;
	let text: string;
	try {
		text = readFileSync(f, "utf8");
	} catch {
		return; // not on this machine
	}
	assert.ok(retryablePerMinuteQuota(text));
});
