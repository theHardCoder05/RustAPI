## How to debug Rust project

Install Rust extention
Install DDLB extension
Create launch.json file in VScode
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            "program": "${workspaceFolder}/my-jira-cli/target/debug/my-jira-cli",
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}