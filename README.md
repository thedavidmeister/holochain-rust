# Holochain in Rust
This is a beginning and architecture proposal of a second iteration of
Holochain build in Rust with the intention to have
1. some modules be compiled to WebAssembly to be reused in Holo's front-end part and
2. be able to use a somewhat mature WebAssembly interperter like [wasmi](https://github.com/paritytech/wasmi) for a new type of Ribosome.

## Build
First [install Rust](https://www.rust-lang.org/en-US/install.html) and then
just run

```
cargo build
```

inside this repository.
Find the executable in ```target/debug/holochain-beta```.

To run the tests (which are in ```src/lib.rs```) just say

```
cargo test
```

## Architecture
I've tried to resemble Redux in Rust and looked at [this code](https://github.com/rust-redux/rust-redux).

instance::Instance has a state::State which is the one global state with
sub-state slices for each module which are defined in each module respectively
(see src/agent/mod.rs, src/network/mod.rs and src/nucleus/mod.rs) and put
together in src/state.rs.

State is only read from the instance

```rs
instance.state().nucleus().dna()
```

and mutated by dispatching an action:

```rs
let entry = Entry{...};
instance.dispatch(state::Action::Agent(Commit(entry)));
```

Instance calls reduce on the state with the next action to consume:

```rs
pub fn consume_next_action(&mut self) {
    if self.pending_actions.len() > 0 {
        let action = self.pending_actions.pop_front().unwrap();
        self.state = self.state.clone().reduce(&action);
    }
}
```

The main reducer creates a new State object and calls the sub-reducers:

```rs
pub fn reduce(&mut self, action: &Action) -> Self {
    State {
        nucleus: ::nucleus::reduce(Rc::clone(&self.nucleus), action),
        agent: ::agent::reduce(Rc::clone(&self.agent), action)

    }
}
```

The module 'state' defines an action type (enum state::Action) that has values for
each sub-module. The modules define their sub-actions themselves and provide
their own sub-reducer function that handles those action types.

Since sub-module state slices are included in state::State as counted references (Rc\<AgentState>) the sub-module reducers can choose if they have the new state object (that the reducer returns) reference the same old sub-state slice (when the action did not affect the sub-state for instance) or if they clone the state, mutate it and return a different reference.

In module agent:

```rs
pub fn reduce(old_state: Rc<AgentState>, action: &_Action) -> Rc<AgentState> {
    match *action {
        _Action::Agent(ref agent_action) => {
            let mut new_state: AgentState = (*old_state).clone();
            match *agent_action {
                Action::Commit(ref entry) => {

                }
            }
            Rc::new(new_state)
        },
        _ => old_state
    }
}
```

With every module handling its state which is read-only for everything else and providing actions to be created from anywhere else that are processed through the reducer hierarchy I hope to decouple modules effectively. Actions being logged make already for a great debugging tool, if that is not enough, the state history could be stored and in a future debugging tool even switched back and forth (time-machine debugging for Holochain :D).