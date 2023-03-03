my contribution to the ChatGPT craze.

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
```
