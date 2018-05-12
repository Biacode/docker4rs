# docker4rst

#### CI status

[![Build Status](https://travis-ci.org/Biacode/docker4rs.svg?branch=master)](https://travis-ci.org/Biacode/docker4rs)

## macos solution to access docker remotely

`docker run -d -v /var/run/docker.sock:/var/run/docker.sock -p 127.0.0.1:2375:2375 bobrik/socat TCP-LISTEN:2375,fork UNIX-CONNECT:/var/run/docker.sock`
