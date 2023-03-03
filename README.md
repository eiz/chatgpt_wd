my contribution to the ChatGPT craze.

Install:

```
cargo install chatgpt2py
```

Put your OpenAI API key in a text file called `~/.openai`

Usage:

```
eiz@eiz13k:~$ gpt print all the sysctls from /proc/sys | python
['abi', 'debug', 'dev', 'fs', 'fscache', 'kernel', 'net', 'sunrpc', 'user', 'vm']
eiz@eiz13k:~$ gpt print all the sysctls from /proc/sys, recursively | python
/proc/sys/abi/vsyscall32
/proc/sys/debug/exception-trace
/proc/sys/debug/kprobes-optimization
/proc/sys/dev/raid/speed_limit_max
/proc/sys/dev/raid/speed_limit_min
/proc/sys/dev/scsi/logging_level
/proc/sys/dev/tty/ldisc_autoload
/proc/sys/fs/aio-max-nr
/proc/sys/fs/aio-nr
...
eiz@eiz13k:~/chatgpt2py$ gpt print the first 10 prime numbers | python
2
3
5
7
11
13
17
19
23
29
eiz@eiz13k:~$ gpt automate updating an ubuntu system | python
[sudo] password for eiz:
```
