#!/usr/bin/env python3
import subprocess
MESSAGE_FLAG = "BOT MESSAGE: AUTO BUMP VERSION"


def shell_call(cmd:str):
    s = subprocess.run(cmd, shell=True, capture_output=True).stdout
    return s.decode("utf-8").strip()


def main():
    branch_name = shell_call("git branch --show-current")
    if branch_name not in ['master'] and branch_name != 'bump':
        print("Current branch  ({})  is not for release. Exiting".format(branch_name))
    commit_log = shell_call("git log --name-status -1")
    if MESSAGE_FLAG in commit_log:
        print("Last commit is generated by bot. Exting")


main()