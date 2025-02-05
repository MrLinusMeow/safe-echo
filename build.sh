rustc main.rs -o a.out
sudo setcap cap_net_bind_service=+ep a.out
