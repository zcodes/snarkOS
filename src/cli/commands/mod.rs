// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.
//
// pub mod client;
// pub use client::*;
//
// pub mod miner;
// pub use miner::*;
//
// use structopt::StructOpt;
//
// #[derive(StructOpt, Debug)]
// #[structopt(name = "snarkos", author = "The Aleo Team <hello@aleo.org>", setting = structopt::clap::AppSettings::ColoredHelp)]
// pub struct CLI {
//     /// Enable debug mode
//     #[structopt(short, long)]
//     pub debug: bool,
//
//     /// Enable verbose mode
//     #[structopt(short, long, parse(from_occurences))]
//     pub verbose: u8,
//
//     #[structopt(subcommand)]
//     pub command: Command,
// }
//
// #[dervie(StructOpt, Debug)]
// pub enum Command {
//     #[structopt(name = "client")]
//     Client(Client),
//
//     #[structopt(name = "miner")]
//     Miner(Miner),
//     // #[structopt(name = "update")]
//     // Update(Update),
// }
//
// impl Command {
//     pub fn parse(self) -> anyhow::Result<String> {
//         match self {
//             Self::Miner(comand) => comand.parse(),
//             Self::Client(command) => command.parse(),
//             // Self::Update(command) => command.parse(),
//         }
//     }
// }
