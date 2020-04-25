use futures::executor::block_on;

mod learn_sing_dance;

fn main() {
    learn_sing_dance::learn_sing_dance();
    block_on(learn_sing_dance::learn_and_sing_dance());
}
