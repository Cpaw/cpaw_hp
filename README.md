# cpaw_hp

## Setup

```
$ git clone https://github.com/cpaw/cpaw_hp
$ cd cpaw_hp
$ emacs .env # Please append about CPAW_TOKEN, CPAW_SECRET
$ ./scripts/setup_db.sh # this command need to run server at first
$ diesel migration run
$ ./scripts/insert_dummy_data.sh # if you wanna develop
$ cargo build --release
```

## Deploy

* nginx
* supervisior

### nginx

```
$ emacs /etc/nginx/site-enabled/cpaw_hp.conf
server {
    location / {
        proxy_set_header X-Real-IP $remote_addr;
		proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
		proxy_set_header Host $http_host;
		proxy_set_header X-NginX-Proxy true;
		proxy_pass http://localhost:3000;
		proxy_redirect off;
    }
}
$ sudo nginx -s reload
```

### supervisor

```
$ emacs /etc/supervisor/conf.d/cpaw_hp.conf
[program:cpaw_hp]
directory=/home/ubuntu/cpaw_hp/                     # example
command=/home/ubuntu/cpaw_hp/target/release/cpaw_hp # example
autostart=true
autorestart=true
stderr_logfile=/var/log/cpaw_hp.err.log
stdout_logfile=/var/log/cpaw_hp.out.log
$ sudo service supervisor start
```
