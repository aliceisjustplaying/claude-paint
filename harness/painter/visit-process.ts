import { spawn } from "node:child_process";

const [pi, ...args] = process.argv.slice(2);
const child = spawn(pi, args, { stdio: ["ignore", "inherit", "inherit"], detached: true });
let stopping = false;
let escalation: ReturnType<typeof setTimeout> | undefined;

function kill(signal: NodeJS.Signals) {
	if (!child.pid) return;
	try {
		process.kill(-child.pid, signal);
	} catch {
		child.kill(signal);
	}
}

function stop() {
	if (stopping) return;
	stopping = true;
	kill("SIGTERM");
	escalation = setTimeout(() => kill("SIGKILL"), 1000);
}

process.stdin.on("end", stop);
process.stdin.resume();
process.on("SIGTERM", stop);
process.on("SIGINT", stop);
child.on("error", (error) => {
	console.error(error.message);
	process.exit(1);
});
child.on("exit", (code) => {
	clearTimeout(escalation);
	kill("SIGKILL");
	process.exit(stopping ? 1 : code ?? 1);
});
