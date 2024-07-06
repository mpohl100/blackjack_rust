use crate::game::channel_game::{ChannelGame, GameAction, GameInfo};
use std::{option, sync::mpsc as std_mpsc, thread};
use tokio::sync::mpsc;

// Synchronous wrapper
pub struct SyncGame {
    game: ChannelGame,
    rt: tokio::runtime::Runtime,
    action_thread: thread::JoinHandle<()>,
    option_thread: thread::JoinHandle<()>,
    stop_sender_action: std_mpsc::Sender<bool>,
    stop_sender_option: std_mpsc::Sender<bool>,
}

impl SyncGame {
    fn new(
        action_receiver: std_mpsc::Receiver<GameAction>,
        option_sender: std_mpsc::Sender<Vec<GameAction>>,
        do_print: bool,
    ) -> SyncGame {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let (async_action_sender, async_action_receiver) = mpsc::channel::<GameAction>(32);
        let (async_option_sender, mut async_option_receiver) = mpsc::channel::<Vec<GameAction>>(32);
    
        let (stop_sender_action, stop_receiver_action) = std_mpsc::channel::<bool>();
        let action_thread = thread::spawn(move || {
            loop{
                if action_receiver.try_recv().is_ok() {
                    let action = action_receiver.recv().unwrap();
                    async_action_sender.blocking_send(action).unwrap();    
                }
                // sleep 50 ms
                thread::sleep(std::time::Duration::from_millis(50));
                if stop_receiver_action.try_recv().is_ok() {
                    break;
                }
            }
        });

        let (stop_sender_option, stop_receiver_option) = std_mpsc::channel::<bool>();
        let option_thread = thread::spawn(move || {
            loop {
                if async_option_receiver.try_recv().is_ok() {
                    let options = async_option_receiver.blocking_recv().unwrap();
                    option_sender.send(options).unwrap();    
                }
                // sleep 50 ms
                thread::sleep(std::time::Duration::from_millis(50));
                if stop_receiver_option.try_recv().is_ok() {
                    break;
                }
            }
        });

        let game = rt.block_on(ChannelGame::new(async_action_receiver, async_option_sender, do_print));
        SyncGame { game, rt, action_thread, option_thread, stop_sender_action, stop_sender_option }
    }

    fn play(&mut self) {
        self.rt.block_on(self.game.play());
    }

    fn ask_to_play_another_hand(&self) -> bool {
        let result = self.rt.block_on(self.game.ask_to_play_another_hand());
        if !result {
            self.stop_sender_action.send(true).unwrap();
            self.stop_sender_option.send(true).unwrap();
        }
        result
    }

    fn get_game_info(&self) -> GameInfo {
        self.rt.block_on(self.game.get_game_info())
    }
}