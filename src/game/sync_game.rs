use crate::game::channel_game::{ChannelGame, GameAction, GameInfo};
use std::{sync::mpsc as std_mpsc, thread};
use tokio::sync::mpsc;

// Synchronous wrapper
#[allow(dead_code)]
pub struct SyncGame {
    game: ChannelGame,
    rt: tokio::runtime::Runtime,
    action_thread: thread::JoinHandle<()>,
    option_thread: thread::JoinHandle<()>,
    game_info_thread: thread::JoinHandle<()>,
    stop_sender_action: std_mpsc::Sender<bool>,
    stop_sender_option: std_mpsc::Sender<bool>,
    stop_game_info_sender: std_mpsc::Sender<bool>,
}

impl SyncGame {
    pub fn new(
        action_receiver: std_mpsc::Receiver<GameAction>,
        option_sender: std_mpsc::SyncSender<Vec<GameAction>>,
        game_info_sender: std_mpsc::SyncSender<GameInfo>,
        do_print: bool,
    ) -> SyncGame {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let (async_action_sender, async_action_receiver) = mpsc::channel::<GameAction>(32);
        let (async_option_sender, mut async_option_receiver) = mpsc::channel::<Vec<GameAction>>(32);
        let (async_game_info_sender, mut async_game_info_receiver) = mpsc::channel::<GameInfo>(32);

        let (stop_sender_action, stop_receiver_action) = std_mpsc::channel::<bool>();
        let action_thread = thread::spawn(move || {
            loop {
                match action_receiver.try_recv() {
                    Ok(action) => {
                        async_action_sender.blocking_send(action).unwrap();
                    }
                    Err(_message) => {}
                }
                // sleep 50 ms
                thread::sleep(std::time::Duration::from_millis(50));
                let stop_thread = match stop_receiver_action.try_recv() {
                    Ok(true) => true,
                    Ok(false) => false,
                    _ => false,
                };
                if stop_thread {
                    break;
                }
            }
        });

        let (stop_sender_option, stop_receiver_option) = std_mpsc::channel::<bool>();
        let option_thread = thread::spawn(move || {
            loop {
                match async_option_receiver.try_recv() {
                    Ok(options) => {
                        option_sender.send(options).unwrap();
                    }
                    Err(_message) => {}
                }
                // sleep 50 ms
                thread::sleep(std::time::Duration::from_millis(50));
                let stop_thread = match stop_receiver_option.try_recv() {
                    Ok(true) => true,
                    Ok(false) => false,
                    _ => false,
                };
                if stop_thread {
                    break;
                }
            }
        });

        let (stop_game_info_sender, stop_game_info_receiver) = std_mpsc::channel::<bool>();
        let game_info_thread = thread::spawn(move || {
            loop {
                match async_game_info_receiver.try_recv() {
                    Ok(game_info) => {
                        game_info_sender.send(game_info).unwrap();
                    }
                    Err(_message) => {}
                }
                // sleep 50 ms
                thread::sleep(std::time::Duration::from_millis(50));
                let stop_thread = match stop_game_info_receiver.try_recv() {
                    Ok(true) => true,
                    Ok(false) => false,
                    _ => false,
                };
                if stop_thread {
                    break;
                }
            }
        });

        let game = rt.block_on(ChannelGame::new(
            async_action_receiver,
            async_option_sender,
            async_game_info_sender,
            do_print,
        ));
        SyncGame {
            game,
            rt,
            action_thread,
            option_thread,
            game_info_thread,
            stop_sender_action,
            stop_sender_option,
            stop_game_info_sender,
        }
    }

    pub fn play(&mut self) {
        self.rt.block_on(self.game.play());
    }

    pub fn ask_to_play_another_hand(&mut self) -> bool {
        let result = self.rt.block_on(self.game.ask_to_play_another_hand());
        result
    }

    pub fn cleanup(self) {
        self.stop_sender_action.send(true).unwrap();
        self.stop_sender_option.send(true).unwrap();
        self.stop_game_info_sender.send(true).unwrap();
    }
}
