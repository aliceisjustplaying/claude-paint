import json
import sys


def executables(lines):
    for line in lines:
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        if not isinstance(message, dict) or message.get("reason") != "compiler-artifact":
            continue
        executable = message.get("executable")
        if executable is None:
            continue
        target = message.get("target")
        profile = message.get("profile")
        if (not isinstance(executable, str) or not executable
                or any(c in executable for c in "\t\r\n")
                or not isinstance(target, dict)
                or not isinstance(target.get("kind"), list) or not target["kind"]
                or not all(isinstance(k, str) and k and not any(c.isspace() for c in k)
                           for k in target["kind"])
                or not isinstance(target.get("name"), str) or not target["name"]
                or any(c.isspace() for c in target["name"])
                or not isinstance(profile, dict) or type(profile.get("test")) is not bool):
            raise ValueError("invalid compiler-artifact executable")
        yield f'{target["kind"][0]} {target["name"]}{" test" if profile["test"] else ""}\t{executable}'


if __name__ == "__main__":
    try:
        with open(sys.argv[1]) as stream:
            for executable in executables(stream):
                print(executable)
    except ValueError as error:
        sys.exit(str(error))
