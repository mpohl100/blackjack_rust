[33mcommit 7733f29bd08cc3519942ea487f38bba007072001[m[33m ([m[1;36mHEAD[m[33m -> [m[1;32mmain[m[33m)[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Jul 14 12:40:29 2024 +0000

    Cargo.lock updates

[33mcommit bc65f72e5c3f8f00454288583d4316f39354071a[m[33m ([m[1;31morigin/main[m[33m)[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Jul 14 07:30:55 2024 +0000

    cargo clippy

[33mcommit 6772914e4e88327c424574a8fbd76c8c0f0e41c1[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Jul 14 07:28:14 2024 +0000

    cargo fmt

[33mcommit f5a10e1b0d7f1a52253797bbb422fff1db52b455[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Jul 14 07:27:54 2024 +0000

    implemented enhancements of the ratatui game

[33mcommit b5733d701af53eeaf67ff6c0f19d61459f430dd2[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Jul 11 15:17:24 2024 +0000

    made multiple hands possible in game_info. Dealer is now played last

[33mcommit 41515201f70844f4ca0dbda4cce1e8909b85a084[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 19:39:36 2024 +0000

    fixed clippy warnings

[33mcommit e4a9c390436eb17219078f4161ff4178fddfb530[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 19:33:20 2024 +0000

    cargo fmt

[33mcommit 231edba1e656f8566aa943fb031befbfd302f43f[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 19:33:05 2024 +0000

    fixed tests

[33mcommit 765c7295a259aac87e595ef911ee56608a22272e[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 19:28:30 2024 +0000

    made options visible

[33mcommit 4990f8c76fcd30fe67c7688e06e5991b7dc54de2[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 19:18:45 2024 +0000

    bugfix: splitting picture cards did not work

[33mcommit ba9e88bf71a6e95e11433de740c157e8e4b2ad6f[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 18:54:01 2024 +0000

    made blackjack game not do weird things

[33mcommit c2c95beaf5f6e64c0290f78acc027b808c5dcc2b[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 18:31:47 2024 +0000

    fixed dead lock after one hand

[33mcommit f1009fe9691f9bf37384ce6f10286547dbad0add[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 18:07:35 2024 +0000

    removed dead lock

[33mcommit 0ffde3e5585b55e7aae4444371621b0fabad62e6[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 18:05:00 2024 +0000

    reset game state after every hand

[33mcommit 4b4b72dd9895467d13abbebac55ab59901ef5000[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 17:51:31 2024 +0000

    make game_optional

[33mcommit 62bc9088efc5d851873fe0031fd1b00433b545ca[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 17:48:03 2024 +0000

    use match also in ratatui main loop

[33mcommit 1d3c4f8df25057b1897bf8d79543893b8af57d7c[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 17:42:20 2024 +0000

    use match statement to receive data

[33mcommit 3008e655891b8d8ea2b839b03e43b000d1dbafef[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 17:16:51 2024 +0000

    removed some warnings

[33mcommit b83e7f00dcf7228f728526645314963a6f995ff2[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Jul 8 17:05:58 2024 +0000

    render raw game layout without data

[33mcommit 5d91044e74753ee05ebc5fff3930596c69e827c8[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Jul 7 18:38:42 2024 +0000

    got project to build again. route for game_info_sender is available

[33mcommit 5f5a9872eac26eae7876c29e2745d515514df6bc[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Jul 7 16:50:18 2024 +0000

    made play_blackjack_hand_new use hand data trait and async

[33mcommit 032b51045f29c5eb9e391ab9e552bd9895614dae[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Jul 7 16:33:14 2024 +0000

    set active hand at the right time

[33mcommit cc0baeb6f0e795491568899dad8202e0d8c8640b[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Jul 7 16:28:53 2024 +0000

    introduced get_player_hand_new

[33mcommit 09620c214a47d4db04ee10bcf99567a6db12e04b[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Jul 6 15:12:53 2024 +0000

    don't synchronously block for options before render loop

[33mcommit 10e964493782d0fd7b679f40ac79d6014cf95699[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Jul 6 15:03:25 2024 +0000

    use sync sender

[33mcommit ebd10a0aaf02a5b11b2dd5a04a339d1acb760512[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Jul 6 14:52:06 2024 +0000

    use sync game in play_ratatui

[33mcommit 61ae580c9704bc4627df5a2a5c9ff1acf410893f[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Jul 6 14:41:42 2024 +0000

    implemented sync game

[33mcommit 17ad6f43aacbcd9b4977321ccf714a65d9e35950[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Fri Jul 5 15:25:58 2024 +0000

    optionally println in channelgame

[33mcommit 171d154aaa075493fdd789a8416aa5047cf0d002[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Fri Jul 5 15:15:53 2024 +0000

    first draft of running blackjack game

[33mcommit 28249501cc4ea654fa45a2befd534e96cc56fe95[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Fri Jul 5 15:00:40 2024 +0000

    started implementing the ratatui game with the channel game

[33mcommit 330d62a2fa92bbd94442737b70d5c4c89f617978[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Jul 4 14:29:04 2024 +0000

    first blackjack layout

[33mcommit dc49ad47ffa358fac9727b1f32c65bc21fd24b96[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Jul 4 14:13:34 2024 +0000

    initialize ratatui game

[33mcommit 7b70d7fd4d9f3bc4b9dcd3ebd1ec8d92a149d64b[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed May 1 09:43:01 2024 +0000

    fix clippy warnings

[33mcommit 7428496ec5d21d70a1d3504a8d4a6ef00beb3395[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed May 1 09:42:19 2024 +0000

    cargo fmt

[33mcommit 458075ec1f44dee87ab824fafdda936a664c90e3[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed May 1 09:41:47 2024 +0000

    minor bugfixes

[33mcommit d8be55115ec3d1ad058cac99432fbe36d24d5a72[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed May 1 09:25:09 2024 +0000

    try to implement stopping of game

[33mcommit fc31ae4a8ee99d1da1352f7cc9c98e0723f70500[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed May 1 09:15:38 2024 +0000

    implemented first draft of get_draw, get_double_down and get_split

[33mcommit 5f7d22559e3aa323b0301fe17c265e73a96f041f[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed May 1 08:44:45 2024 +0000

    parse command line input in channel game and convert it to a game action

[33mcommit 30fe3073d6f396e318ae366c26fbd95df40f6be1[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 17:04:26 2024 +0000

    fixed last clippy warning

[33mcommit 457f7da59b3daa64c15b3cd7e78b93649354eaf5[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 17:03:16 2024 +0000

    forgot to save

[33mcommit a4815cf551bfc649a05da9c67f11fead4c5f0481[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 17:00:52 2024 +0000

    fixed some clippy warnings

[33mcommit a2fe4798f075ef3d913550fd3501d22450187dcd[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 16:52:47 2024 +0000

    cargo fmt

[33mcommit 6c0964b1490979e037cf83165dd246d350eab3f8[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 16:52:03 2024 +0000

    fix build warnings

[33mcommit 7b060da1091a39344da349567e7055f5a1218d1b[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 16:45:04 2024 +0000

    cargo fix

[33mcommit e8dd023cf8ac898470b1b626703e46c939bd0ee8[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 16:37:25 2024 +0000

    await some funcions

[33mcommit 62ea003f65e4fa9de20711a586709c1808437fe1[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 11:00:30 2024 +0000

    add some more awaits

[33mcommit 42273cfe338348fc1eab18a55ceda2f81ced70b3[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 10:43:51 2024 +0000

    make integration test use tokio

[33mcommit 1f29c11cde17771d2bccf5d6457f9367e7ecf9cc[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 10:40:23 2024 +0000

    got project to build

[33mcommit 823fcbdb78403413375980f430827f749d1ca8b2[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 10:26:20 2024 +0000

    use stdrng

[33mcommit 71511f469e4855189149b6294176d7c365763d91[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 10:22:36 2024 +0000

    await new async methods

[33mcommit 8a8f5c9c045def5be581c986624a7ed5f85a155c[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 10:21:01 2024 +0000

    make blackjack strategy trait async

[33mcommit cd6cdddd05a3f09cf337ec41acd8893b08a41c58[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 10:12:35 2024 +0000

    use tokio mutex

[33mcommit d8dc28938ad5bf7fc3baa6573eded3ebbbabb106[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 10:02:28 2024 +0000

    use temporary box

[33mcommit 619512758bbc2756854b397828a3607f8a9fc0b9[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 09:18:42 2024 +0000

    got build errors removed

[33mcommit 36b6ebe6a715742e028c5431a17e6ac5d6bb2c1e[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 09:16:09 2024 +0000

    changed games to use wrapped game

[33mcommit 0733d2ff1a792aa0101b9037c19328d20c43f9f6[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 09:02:53 2024 +0000

    use WrappedGame in play_blackjack_hand

[33mcommit 613474085fb0cf49b13e6759a8dfda60a5a7cdbf[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 08:49:26 2024 +0000

    implemented cloning of dyn BlackjackStrategyTrait to WrappedGame

[33mcommit 2a7b6a704d9fece45953dfbbc536a6e0a8c0301b[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 08:03:42 2024 +0000

    implemented WrappedGame

[33mcommit da43d6f6af8ca338e779f10ca72c87994263c665[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 01:40:29 2024 +0000

    continued getting WrappedStrategy to build

[33mcommit 49b33fd4e738167a5244cbbc49666db6d219671e[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 28 01:21:10 2024 +0000

    introduced wrapped strategy

[33mcommit 909f55a831620fb59d817caa855dac278aba56bb[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 19:40:04 2024 +0000

    don't use dyn BlackjackStrategyTrait in WrappedStrategy

[33mcommit 0258c3c1af0b9f5946bb4591c80301f300e1b56d[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 19:33:06 2024 +0000

    implemented WrappedStrategy

[33mcommit 63715c4e568d7550092e186537c932738a15ae3b[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 19:17:41 2024 +0000

    wrapped deck now implements all methods of deck trait

[33mcommit 4b7a7b86ad7539a0b6e57d76f9182f32ae0b0e2d[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 18:54:18 2024 +0000

    made async play_blackjack_hand compile

[33mcommit 31b08408efc44624341af6938542d38fcf1757b4[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 18:49:39 2024 +0000

    continued removal of threadpool

[33mcommit 7c314ce608fe9f1c32e50bfdd7c5370eac861b66[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 18:37:34 2024 +0000

    removed threadpool crate

[33mcommit 0b0ae5e7cf22b703ee26e705e51999e64f6facc1[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 17:33:56 2024 +0000

    got some more to build

[33mcommit 5d872631520727098e7ff8dc81709bb6454479b1[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 17:11:09 2024 +0000

    finished introducing WrappedDeck

[33mcommit ff6ba549a5e79180d50ea066cfd011253b681ada[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 16:56:35 2024 +0000

    introduce wrapped deck

[33mcommit b3c8303da80110fc89e09dfeee692fa26b0bb5fc[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 16:27:56 2024 +0000

    making deck async continued

[33mcommit 363af5cf2aec1f3b6f743cf579437fc048ee5f57[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 16:08:20 2024 +0000

    make BlackjackGame trait async

[33mcommit 699e221d4e9385a833454607a704cb7a633636a5[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 27 15:52:54 2024 +0000

    await choice

[33mcommit ecdac99abc4c43d8579ec92f18300a8cfabab3a8[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Fri Apr 26 17:16:13 2024 +0000

    implement game loop in main thread and pass senders and receivers

[33mcommit c5f960f3f4bfd164f3d083bf2385cb5a8cc72e71[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Fri Apr 26 17:00:09 2024 +0000

    start channel game in newly spawned thread

[33mcommit 94035d1b7093b2a3273df62d7bbd8c94d2535ed0[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Fri Apr 26 16:26:39 2024 +0000

    rename to play_cli_game

[33mcommit 541fcf519830a706dae1f3c1f6a9edb0d16a08f3[m[33m ([m[1;31morigin/sync_version[m[33m)[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 18:46:58 2024 +0000

    cargo fix

[33mcommit 7170b4c8c6db85f10b9cb610c7d0460720a58a02[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 18:36:54 2024 +0000

    make blackjack game trait functions take &mut self

[33mcommit 041e124701eadf8aa6cb87f783d04e78f628e493[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 18:17:47 2024 +0000

    renamed to ChannelGame

[33mcommit 0d7628da930a54af5ce2e9563f9037a890f3aafd[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 18:15:22 2024 +0000

    cargo fmt and cargo clippy

[33mcommit a7601af5a0fdb9e195d5cd2fa616237baa5187cf[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 18:13:10 2024 +0000

    borrow game data as mutable

[33mcommit dcc067633e591ac28cbccff5d20220fafd2a06d5[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 18:11:20 2024 +0000

    introduce game data struct to decouple lifetimes

[33mcommit b7f62bda24f283699ae5724d73c42143806a43f9[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 18:04:42 2024 +0000

    some progress on getting the gamestate mutable

[33mcommit 216d0cb478b0f51c7abce49a03521f1678852768[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 17:31:01 2024 +0000

    start channel_game by copy pasting cli game

[33mcommit 9a1546ff01f324b96957deb133ce3bf92c47f3d9[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 16:55:48 2024 +0000

    cargo clippy --fix

[33mcommit 445636c9459d3cf4d52f0ba1e77846881b7feac5[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 16:55:08 2024 +0000

    cargo fmt

[33mcommit 080731dc3be287b0ba1a7340b94ac30fcab29aa9[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 16:54:48 2024 +0000

    implement upcasting myself

[33mcommit f074bf1b284c223e3e435f5a37a1435214a7768e[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 25 16:37:51 2024 +0000

    added BlackjackGame trait

[33mcommit 4007c3076c19e2510c913698ee4cc06f5cfde33a[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Apr 22 19:31:05 2024 +0000

    underscore for dead code warning

[33mcommit 62171cc58b66ea1b83f4e085eba564a6275a9211[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Apr 22 19:17:35 2024 +0000

    cargo clippy --fix

[33mcommit c3beeca292983566dcd50f5a779051e2f4bfa5a0[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Apr 22 19:16:43 2024 +0000

    fixed clippy warning about bearer token parsing

[33mcommit 26cc9141d42c66a442919c79f429882bc26c9732[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Apr 22 19:05:19 2024 +0000

    cargo fmt

[33mcommit 1465fbb0db6c01d440de68c79b1e1a9ee9dca2a5[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Apr 22 19:04:44 2024 +0000

    implemented webserver which does nothing

[33mcommit bac50814c6b798d73b398243cc51195659c80e69[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Mon Apr 22 20:40:35 2024 +0200

    make player_hands plural

[33mcommit 37a64a282196ac47248fb11ccecf75e5b69f9bae[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Apr 21 08:34:18 2024 +0000

    renamed openapi file

[33mcommit 10a08270df667853f4825d9063a7b8d461ef22b0[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 20 20:03:44 2024 +0000

    specify play endpoint

[33mcommit efa5d9133aa8f1f83f32135f27b329c76cfe5c71[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 20 19:52:37 2024 +0000

    remove game request

[33mcommit 2ec804294d6f49838543ac543eca2910fa42b7a6[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 20 19:51:35 2024 +0000

    specify creation and deletion of blackjack game

[33mcommit 0195b01e81674a92cc7c6b8af63246ff4aeef095[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 20 19:13:09 2024 +0000

    add swagger viewer

[33mcommit 4cb86972355f02c71329a6096ab29bf7ab94d926[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 20 07:00:44 2024 +0000

    cargo fmt

[33mcommit 1cab4585ce28c49b49aca2a15322c3ef95bf965a[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 20 06:56:51 2024 +0000

    rename variable from percentages to decisions

[33mcommit dfeae90915bcbeda91586c6811014e4e33db0db0[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 18 16:44:28 2024 +0000

    cargo fmt

[33mcommit b677e17d8e9fc4f80436bb3f1ac50f8a9f3c9417[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 18 16:42:19 2024 +0000

    linter checks

[33mcommit b010407bb472e756cdcf0422549ac776ce1d4c79[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Thu Apr 18 16:40:11 2024 +0000

    implemented feedback whether it was the correct decision

[33mcommit 41f8dbac58d4eb7210e006f6941146804083ec12[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed Apr 17 19:10:32 2024 +0000

    cargo clippy fix

[33mcommit e0e771e3b8b1eae8de3c6dd3c3e0c83bacab1d7f[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed Apr 17 19:09:55 2024 +0000

    cargo fix

[33mcommit 7557d9e21c37484a3222f6a71fdc359b7c8ce0ae[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed Apr 17 19:08:46 2024 +0000

    first try of game

[33mcommit 46ae6e6fdcbe06c983da5d6e124c35ce22c6cadd[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed Apr 17 18:35:19 2024 +0000

    first draft of game

[33mcommit c5f690545a5f8131df507244727b9090ee9fa86c[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed Apr 17 15:13:38 2024 +0000

    correct tests

[33mcommit 2e1eaad73ab960c4e18b660a5055980cf7a91a9e[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed Apr 17 15:09:25 2024 +0000

    activate clippy with warnings in the ci

[33mcommit 764eef251f657ce1b7fe3124c0010038282bc0d2[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed Apr 17 15:08:43 2024 +0000

    removed last cargo clippy warning

[33mcommit b4128f9d17f12949454431d68e86a175e60572b1[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Wed Apr 17 13:41:22 2024 +0000

    fix clippy warning to use cmp and match

[33mcommit 2de628f9d4f6d641771a06176163d69ad01af4cd[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 19:47:53 2024 +0000

    cargo clippy improved the speed

[33mcommit d1d05cd6be30dbc8ec6ef6b7a135f49232162346[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 19:46:02 2024 +0000

    cargo clippy warning about vec!

[33mcommit 7e3b62b8ab366e0d82d03fa2f05f9b5231e736ad[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 19:43:17 2024 +0000

    cargo clippy warning

[33mcommit 540ee34ecd3a0ded0ccccfcfaadd10ae87271c83[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 19:20:15 2024 +0000

    cargo fmt

[33mcommit 85e2418fda3436760b7b623537d693466ad511ea[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 19:19:58 2024 +0000

    removed mod statements

[33mcommit fea34ae0ba59c9d3db06f60217c76e713db8a9b5[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 19:06:08 2024 +0000

    cargo fmt

[33mcommit 99701942754ecc03db053c88ff8753b4362146b4[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 19:05:51 2024 +0000

    factor out counted module

[33mcommit a03b0a6cff55ba64c15c8ca9ec4191616f7db70c[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 19:00:56 2024 +0000

    consider analysis in test

[33mcommit ca97d32e901e7c6b34d188ec1b1c3824ad06f5b0[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 19:00:04 2024 +0000

    analysis subdirectory

[33mcommit 33082edcd5412ea8b8803f926eb8ddcc85da5d83[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 18:52:15 2024 +0000

    renamed blackjack strategy to blackjack strategy combined vec

[33mcommit 6cf61bbf7f6970bd169742bf4e2023db9a8b0334[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 18:49:59 2024 +0000

    separated blackjack strategy combined ordered hash map

[33mcommit 2f5df869b96f9b25b5edbce5fe4da54742dbb8ce[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 18:45:46 2024 +0000

    separated blackjack strategy combined hash map

[33mcommit 908f3506310b41690f805f910193a4cba9a50879[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 18:40:19 2024 +0000

    separated blackjack strategy vec

[33mcommit 9cec2c7cd1d613c5ed910c7e0cecd89c77e6bcac[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 18:35:38 2024 +0000

    separated counted blackjack strategy

[33mcommit c6f2cbbb0cf8dfa676f51f3755b6d57d0bb84621[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 18:26:45 2024 +0000

    introduced blackjack strategy map

[33mcommit e0ff8b5f514fefadced7ec9556d6d87ec6f6612a[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 18:15:02 2024 +0000

    cargo fmt

[33mcommit 42d405edb5aed887360b906f764f3f7a2c025159[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 18:08:33 2024 +0000

    introduced strategy directory

[33mcommit bea892cbe7f89802d4639b7d02de123aa5491681[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 15:36:10 2024 +0000

    no clippy warnings as of yet

[33mcommit ecb05369bdb66707a475163a8e45a077f78c426c[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 15:30:47 2024 +0000

    cargo fmt

[33mcommit 8ef7628558147c9715caa2650d6bf4d887512027[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 15:25:13 2024 +0000

    add formatting and linting

[33mcommit 012399dcdb10b34212088ea68112bf7e9439c64d[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 15:19:16 2024 +0000

    cargo clippy --fix

[33mcommit 267514a12a734754e9d6eb3f74146c273eefca9c[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 15:18:30 2024 +0000

    use combined blackjack strategies in binaries for performance

[33mcommit 45d45e9a9173cd6590635581cd128c864ef78b9b[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 15:08:41 2024 +0000

    added blackjack strategy ordered hash map and vec

[33mcommit 1c9f37623c3c613d0ee89bd21f39f5b521ee36c1[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 14:59:55 2024 +0000

    added blackjack strategy combined hash map

[33mcommit d2481b80609dfe9afc66b9c9eddd810ab4e14fcd[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 04:53:13 2024 +0000

    cargo clippy --fix

[33mcommit c86cb4cef406a3c746749f3603fe24ae4bdabe94[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 04:51:09 2024 +0000

    introduced game_situation struct

[33mcommit be9925dfd72fc148aebb65c4b5e4a300a05548b6[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Apr 16 04:24:05 2024 +0000

    added blackjack integration test

[33mcommit 2918aa16070a700a5cff318eb0156d6a5f3cf836[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 13 06:05:58 2024 +0000

    removed unused usings in tests

[33mcommit fd40e3ad6042532a1c74d28a9306ed149e28ab5b[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 13 06:05:15 2024 +0000

    used Rank and Suit in test modules

[33mcommit c35f1aac04e22708692ecc6d17d97449e1902565[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 13 05:56:52 2024 +0000

    cargo clippy --fix

[33mcommit c942726a1d9eb6dac14a654a695271aec665e788[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 13 05:51:01 2024 +0000

    removed suit getter

[33mcommit 894a967514f63d80f1d9cc98fc88d06263f0b472[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 13 05:47:10 2024 +0000

    cargo fix bin play_counted

[33mcommit 704733f7da359149156ee0a849a2d955d6f9a8e0[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 13 05:45:41 2024 +0000

    cargo fix play_normal

[33mcommit b5410d582baab1bed29ae8bebfc430dfc606a176[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sat Apr 13 04:52:43 2024 +0000

    pass thread_pool from the outside

[33mcommit 7cbf76506a1e1e90f7ad6576c7c1ea2f106cd553[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Fri Apr 12 20:42:51 2024 +0200

    use rust devcontainer with bullseye

[33mcommit 5777baf71e9db68b226872bfc4a6b5a376d87bc6[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Fri Apr 12 18:55:30 2024 +0200

    add build and test yaml

[33mcommit 3385ed5e65ea3d8fb6ad191869c71631edefa4fd[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jun 17 10:44:11 2023 +0200

    correction: there was no bug. corrected test assertion

[33mcommit c08978acf66508ca54ba81a53de04415844584d1[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jun 17 10:38:44 2023 +0200

    found incorrect count bug in counted deck

[33mcommit 5f90ca764e6bb024c78fa3369038d481f7e2bbc6[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Fri Jun 16 17:37:10 2023 +0200

    added tests for eight_decks

[33mcommit 0de705af0de78a744bfec4e0fbe388a21b6ef5c7[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Fri Jun 16 17:20:04 2023 +0200

    fixed bug in deal_card of counted_deck. Now the removed cards are considered correctly.

[33mcommit 859ba047ad587db2513af6fc971cd9df16ee2a2c[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Fri Jun 16 17:01:17 2023 +0200

    added tests for counted deck

[33mcommit 0d061f502a2ba5b33205eed22cc947bc675f570b[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Thu Jun 15 17:30:43 2023 +0200

    added tests for blackjack_situation.rs

[33mcommit ad69869d89232571ca8c48519f82f06a908490a0[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Thu Jun 15 08:05:08 2023 +0200

    added tests in blackjack_points. deleted unnecessary file

[33mcommit 7942cc014c68ca2d8858939457f2f97840b1ac92[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Thu Jun 15 07:20:09 2023 +0200

    added tests in hand.rs

[33mcommit 7fdf61e420347c6398bcd0d0b0f8a1996719175f[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Wed Jun 14 19:21:34 2023 +0200

    added unit tests in card.rs

[33mcommit f4240d5c7f8cbebf74e849f3647fbd8cc2c11f0d[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Jun 13 21:47:29 2023 +0200

    add logging for cards

[33mcommit 808b530c7bc3eca2cbac520234c4e4346023e72e[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Tue Jun 13 18:46:08 2023 +0200

    Use CountedDeck(0) until we find out why with EightDecks the software wins

[33mcommit b8b96e72c635cb9ef5e1cad99c7fec824f0f741a[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Jun 13 18:22:21 2023 +0200

    parallelize optimize_counted method

[33mcommit 8fdd4ff918bf89feba8681c9095212362b6b07ca[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Jun 13 18:07:42 2023 +0200

    measure time of strategy calculation

[33mcommit 8c33429bb428478d918b8b795cf31c8f3dd5b82a[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue Jun 13 17:59:35 2023 +0200

    introduced strategy configuration and play configuration. removed main. all four binaries build now

[33mcommit 07ba6adce957bc4d3b4440bad45ee8899e012597[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jun 10 19:59:42 2023 +0200

    parallelized split

[33mcommit b15f9d4dc1fd357e52368d637397aa29fc8a7e69[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jun 10 19:54:47 2023 +0200

    parallelized double down

[33mcommit a9a68122c08083406d9a3c0508c329e410c2d7dc[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jun 10 17:19:52 2023 +0200

    introduced Send trait and static lifetime tracker. Now works

[33mcommit 10430a89cd4dd583dec73127ff460f13ab2b119a[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jun 10 17:13:09 2023 +0200

    Cannot send BlackjackStrategyType between threads safely error message

[33mcommit 134da5cc2227463ddabe663a3eefa8fad9131efc[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Fri Jun 9 13:12:26 2023 +0200

    added combine and dump API to BlackjackStrategyTrait to be able to combine the BlackjackStrategy after parallelizing its calculation

[33mcommit ee6b04cea708e7efce77e2b447dc131de7a740ac[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Thu Jun 8 15:12:54 2023 +0200

    factored out function calculate_draw which can be later used to parallelize

[33mcommit c538b9a95b7602f2e1c1459dd85a513ee366fa8d[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Wed Jun 7 20:08:56 2023 +0200

    put all hand situations into buckets with the same dealer card. Can now sequentially compute all the results

[33mcommit c837da06e546e72d0144430a4a3588883fdd2046[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Fri Apr 12 16:59:02 2024 +0200

    add github copilot

[33mcommit 3cb0989be0db6359c88a767008b60d35c3d83ea6[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Fri Apr 12 16:56:22 2024 +0200

    add devcontainer file

[33mcommit 68974ff23dae82f034ba073b52935504c21ffa74[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue May 30 19:53:54 2023 +0200

    Update README.md
    
    use backticks

[33mcommit 29f4be549696c4994657de22a231312adb131c4b[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Tue May 30 19:52:16 2023 +0200

    Create README.md
    
    describe the four binaries.

[33mcommit faf7a88d56c3f82e9a0352a8b72442eb2bc3f451[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sun May 28 18:09:11 2023 +0200

    added commandline parameters support to the binaries. Now I can specify how many hands to play from the commandline

[33mcommit fa90f26c3ddcdb3b9f8f02956ad0e32fd6d4a97c[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat May 27 20:54:34 2023 +0200

    removed comment, restructure play_blackjack_hand into more readable if else block for the double down case

[33mcommit 7a6cf00ab846333784b7f855513c78bb5f829ed6[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat May 27 17:33:18 2023 +0200

    added play_counted_performance, only measure playing

[33mcommit f9d4baee590fb5329f30c66a3cbe19d9761880be[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat May 27 17:11:19 2023 +0200

    added performance test

[33mcommit 6c5cfd99558462490a6f657c4b10829f7cdd8341[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat May 27 16:26:10 2023 +0200

    caller now decides which type of BlackjackStrategy is constructed from the library. It is no longer hard coded. Performance Test binary can be implemented now

[33mcommit 43cb9c2706dde89b7d9e0ac8ee5f055b1e7a97d1[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Tue Feb 14 19:57:03 2023 +0100

    use situation::create_all in blackjack_analysis function

[33mcommit a2b61d335bf03a88c6a48d72c460b7cd3ce4311f[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sun Feb 5 18:51:19 2023 +0100

    implement hash for BlackjackSituation and use it in BlackjackStrategy

[33mcommit 15fda32a1be3158859e404403ebd2236198d63ba[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Feb 4 21:29:27 2023 +0100

    make BlackjackStrategyVec public

[33mcommit ae3b7a8c7b6db93bfcbcfd168a9ae874248d6c3d[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Feb 4 20:58:22 2023 +0100

    added BlackjackStrategyVec

[33mcommit f1f05dd005c3f4c9bd13b1e3ee383e088ecef50f[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Fri Feb 3 04:05:58 2023 +0100

    implemented clamping of count for play_counted

[33mcommit 8138eaa43e17a653ab06b6e143f260887688df49[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sun Jan 29 10:41:21 2023 +0100

    implemented CountedBlackjackStragegy and created new binary play_counted
    
    implemented EightDecks

[33mcommit d0feb62fa930dfcf933508b30dcd334587fceb71[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jan 28 20:01:18 2023 +0100

    removed rc now share result via mutable reference to dyn trait of blackjack strategy

[33mcommit a6dd99837701d486d6174c17649d28c63f055c28[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jan 28 19:19:10 2023 +0100

    changed box to rc

[33mcommit aac6ef51e2c55f0ff853d62d07bdbe0fd8cd168a[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jan 28 18:48:42 2023 +0100

    first try to implement a strategy trait

[33mcommit 3bccd0cf1ae9a63d9eee094d638cd8cb34d7a109[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jan 28 14:17:08 2023 +0100

    introduced new binary which plays a million hands according to the strategy. fixed bug with blackjack payoff

[33mcommit 0573001ea8bae7ec762de6cf291cfb67dc4ff075[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jan 28 11:39:43 2023 +0100

    removed logging; corrected bug in Points new method

[33mcommit dcce75e25daa9014e6eaef41668accee0d809bce[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jan 28 10:52:42 2023 +0100

    added debugging logs and Stringable trait

[33mcommit ae3d6c574ef5e2f1844612914c0a7d07f59d6eb4[m
Merge: 576a5c1 515e4db
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Mon Jan 23 19:20:34 2023 +0100

    Merge branch 'master' into main

[33mcommit 515e4dbcf952c1b448de655021d4cd35bbb80ae7[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Mon Jan 23 19:14:02 2023 +0100

    set up software to debug card count 0

[33mcommit 13438ac5484b0ba408fb70fc460ee7b4cca0ddb4[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sun Jan 22 13:55:34 2023 +0100

    removed warnings, implemented missing methods

[33mcommit 5a88015d6badfdd329254c707e8f1bdb6bbd2aef[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Mon Jan 16 19:16:14 2023 +0100

    implemented further methods

[33mcommit 9534cd63f040ec5f99a6211756f55055d87a3233[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Mon Jan 16 19:05:36 2023 +0100

    implemented removal of cards in the new function of deck

[33mcommit d8e8a1c5ee8439b40e976c3cceaafd44bf1ddf43[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Mon Jan 16 18:16:45 2023 +0100

    implemented to_string_mat_2

[33mcommit 0a99e0a06182c209cc68cbe8987c72c06f3cd1c8[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sun Jan 15 18:38:24 2023 +0100

    add score implementation

[33mcommit 576a5c1744b74320849410a473fe0c179609bc5f[m
Author: Michael Pohl <mpohl100@googlemail.com>
Date:   Sun Jan 15 18:04:10 2023 +0100

    Initial commit

[33mcommit 6f12101161027ca43654bba7d2740afcf2dc265a[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sun Jan 15 17:09:40 2023 +0100

    implemented nearly the rest of the blackjack project. Only the connecting middle part and some functions at the bottom are missing

[33mcommit b748cd591019b84cc33122735972599b0335d2cb[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jan 14 16:52:32 2023 +0100

    fixed all build errors

[33mcommit 151bc0b9b0db9c6fcb0ec44f74d9d7b86de7d25a[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jan 14 16:27:39 2023 +0100

    implementing optimize_blackjack function

[33mcommit d4311ce277c69c90f6da01e5e28a3c60c8a1094e[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jan 14 15:22:23 2023 +0100

    generated types for implementation of the blackjack project

[33mcommit a9495fe5cba070d2564240217bbce6becd50ed40[m
Author: Michael Pe <mpohl100@googlemail.com>
Date:   Sat Jan 14 12:16:59 2023 +0100

    initialize blackjack project to hello world
