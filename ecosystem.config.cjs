module.exports = {
  apps: [
    {
      "name": "homeland-frontend",
      "script": "./serve/index.prod.js",
      "cwd": "./frontend",
      "log_date_format": "YYYY-MM-DD HH:mm Z",
      "instances": 2,
      "max_memory_restart": "1000M",
      "merge_logs": true,
      "exec_mode": "cluster",
      "kill_timeout": 5000,
      "ignore_watch": ["node_modules", "logs", "public"],
      "watch_options": {
        "followSymlinks": false
      },
      "error_file": "./logs/pm2-err.log",
      "out_file": "./logs/pm2-out.log",
      "env": {
        "NODE_ENV": "production"
      },
      "args": [
        "--color"
      ]
    }, {
      "name": "homeland-backend",
      "instances": 2,
      "cwd": ".",
      "script": "./backend",
      "output": "/dev/stdout",
      "error": "/dev/stderr",
    }
  ]
}