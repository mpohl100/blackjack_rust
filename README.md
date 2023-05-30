# blackjack_rust
The purpose of this project is to have a personal project where I can get to know the features of the Rust language.
The blackjack library enables the user to deduce a Blackjack strategy table which can then be used in actual playing against the house.
As the house always has an advantage, it is impossible to win. That's why the library is able to use the method of card counting which puts the user in a winning position by a small margin.
A side effect of this library is that the Blackjack strategy table can be implemented via a HashMap, an OrderedMap or a Vec. A conveniance binary is provided in order to compare the three storage technologies.

## play_normal binary
This binary first deduces the optimal blackjack strategy and then plays as many hands as the user specifies via the command line parameter -n.
It is expected that the user ends up a small net loser.
The example call is this:
'''
cargo run --bin play_normal --release -- -n 1000000
'''

## play_counted binary
This binary first deduces the optimal blackjack strategy and uses the popular method of card counting and then plays as many hands as the user specifies via the command line parameter -n.
It is expected that the user ends up a small net winner.
Note that play_counted should take approximately 21 times longer as play_normal as it has to compute the blackjack strategies for decks with counts from -10 to 10.
The example call is this:
'''
cargo run --bin play_counted --release -- -n 1000000
'''


## play_performance binary
This binary calls the same method as play_normal but it uses the four storage technologies HashMap, OrderedMap, ReversedVec and Vec in order to store the blackjack startegy.
The program plays as many hands as the user specifies via the command line parameter -n and outputs the time it took to play as many hands as specifed for each storage technology.
It is expected that the user ends up a small net loser.
The example call is this:
'''
cargo run --bin play_performance --release -- -n 1000000
'''


## play_counted_performance binary
This binary calls the same method as play_counted but it uses the four storage technologies HashMap, OrderedMap, ReversedVec and Vec in order to store the counted blackjack startegy.
The program plays as many hands as the user specifies via the command line parameter -n and outputs the time it took to play as many hands as specifed for each storage technology.
It is expected that the user ends up a small net winner.
Note that play_counted_performance should take approximately 21 times longer as play_performance as it has to compute the blackjack strategies for decks with counts from -10 to 10.
The example call is this:
'''
cargo run --bin play_counted_performance --release -- -n 1000000
'''
