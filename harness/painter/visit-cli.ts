import { criticFromEnv, leaveNote, passagesToTakeUp, visit } from "./visit.ts";

const controller = new AbortController();
for (const signal of ["SIGTERM", "SIGINT"] as const) process.on(signal, () => controller.abort());

try {
	let input = "";
	for await (const chunk of process.stdin) input += chunk;
	const { occasion, note } = JSON.parse(input);
	if (occasion !== "asked" && occasion !== "sitting") throw new Error("invalid visit occasion");
	if (note !== undefined && typeof note !== "string") throw new Error("invalid critic note");
	const critic = criticFromEnv();
	if (!critic) throw new Error("PAINTER_CRITIC is not set");
	const studio = process.cwd();
	if (occasion === "asked" && note?.trim()) leaveNote(studio, critic, note);
	const notes = await visit(studio, critic, occasion, controller.signal);
	process.stdout.write(JSON.stringify({ notes, opening: passagesToTakeUp(studio, true) }) + "\n");
} catch (error) {
	console.error((error as Error).message);
	process.exitCode = 1;
}
