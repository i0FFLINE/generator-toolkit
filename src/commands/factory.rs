// src/commands/factory.rs
use crate::cli::parser::{CliArgs, Commands};
use crate::commands::{
    bcrypt::BcryptCommand, config::ConfigCommand, cuid2::Cuid2Command, nanoid::NanoidCommand,
    password::PasswordCommand, tsid::TsidCommand, ulid::UlidCommand, uuid4::Uuid4Command,
    uuid5::Uuid5Command, uuid7::Uuid7Command, Command,
};
use crate::config::Config;

pub struct CommandFactory;

impl CommandFactory {
    pub fn create(args: CliArgs, config: Config) -> Box<dyn Command> {
        let json = args.json;
        match args.command {
            Some(Commands::Password {
                lower,
                upper,
                digits,
                special,
                extra,
                length,
                repeat,
                reuse,
                exclude_ambiguous,
                strategy,
                count,
            }) => {
                let cfg = config.password.unwrap_or_default();

                let final_lower = lower.or(Some(cfg.lower)).unwrap_or(true);
                let final_upper = upper.or(Some(cfg.upper)).unwrap_or(true);
                let final_digits = digits.or(Some(cfg.digits)).unwrap_or(false);
                let final_special = special.or(Some(cfg.special)).unwrap_or(false);
                let final_extra = extra.or(Some(cfg.extra)).unwrap_or(false);
                let final_length = length.or(Some(cfg.length)).unwrap_or(13);
                let final_repeat = repeat.or(Some(cfg.repeat)).unwrap_or(0);
                let final_reuse = reuse.or(Some(cfg.reuse)).unwrap_or(0);
                let final_exclude = exclude_ambiguous
                    .or(Some(cfg.exclude_ambiguous))
                    .unwrap_or(false);
                let final_strategy = strategy
                    .or(Some(cfg.strategy))
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "retry".to_string());
                let final_count = count.unwrap_or(1);

                Box::new(PasswordCommand::new(
                    final_length,
                    final_lower,
                    final_upper,
                    final_digits,
                    final_special,
                    final_extra,
                    final_repeat,
                    final_reuse,
                    final_exclude,
                    final_strategy,
                    final_count,
                    json,
                ))
            }
            Some(Commands::Config) => Box::new(ConfigCommand::new(json, args.yes)),
            Some(Commands::Uuid4 { count }) => {
                let final_count = count.unwrap_or(1);
                Box::new(Uuid4Command::new(final_count, json))
            }
            Some(Commands::Uuid5 {
                namespace,
                name,
                count,
            }) => {
                let final_count = count.unwrap_or(1);
                let cfg = config.uuid5.unwrap_or_default();
                let final_namespace = namespace.or(cfg.namespace);
                let final_name = name.or(cfg.name);
                Box::new(Uuid5Command::new(
                    final_namespace,
                    final_name,
                    final_count,
                    json,
                ))
            }
            Some(Commands::Uuid7 { count }) => {
                let final_count = count.unwrap_or(1);
                Box::new(Uuid7Command::new(final_count, json))
            }
            Some(Commands::Ulid { count }) => {
                let final_count = count.unwrap_or(1);
                Box::new(UlidCommand::new(final_count, json))
            }
            Some(Commands::Cuid2 { count }) => {
                let final_count = count.unwrap_or(1);
                Box::new(Cuid2Command::new(final_count, json))
            }
            Some(Commands::Nanoid {
                length,
                alphabet,
                count,
            }) => {
                let final_count = count.unwrap_or(1);
                let cfg = config.nanoid.unwrap_or_default();
                let final_length = length.or(cfg.length).unwrap_or(21);
                let final_alphabet = alphabet.or(cfg.alphabet);
                Box::new(NanoidCommand::new(
                    final_length,
                    final_alphabet,
                    final_count,
                    json,
                ))
            }
            Some(Commands::Tsid { count }) => {
                let final_count = count.unwrap_or(1);
                Box::new(TsidCommand::new(final_count, json))
            }
            Some(Commands::Bcrypt { value, rounds }) => {
                let cfg = config.bcrypt.unwrap_or_default();
                let final_rounds = rounds.or(cfg.rounds).unwrap_or(12);
                Box::new(BcryptCommand::new(value, final_rounds, json))
            }
            None => {
                let cfg = config.password.unwrap_or_default();

                let final_lower = args.lower.or(Some(cfg.lower)).unwrap_or(true);
                let final_upper = args.upper.or(Some(cfg.upper)).unwrap_or(true);
                let final_digits = args.digits.or(Some(cfg.digits)).unwrap_or(false);
                let final_special = args.special.or(Some(cfg.special)).unwrap_or(false);
                let final_extra = args.extra.or(Some(cfg.extra)).unwrap_or(false);
                let final_length = args.length.or(Some(cfg.length)).unwrap_or(13);
                let final_repeat = args.repeat.or(Some(cfg.repeat)).unwrap_or(0);
                let final_reuse = args.reuse.or(Some(cfg.reuse)).unwrap_or(0);
                let final_exclude = args
                    .exclude_ambiguous
                    .or(Some(cfg.exclude_ambiguous))
                    .unwrap_or(false);
                let final_strategy = args
                    .strategy
                    .or(Some(cfg.strategy))
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "retry".to_string());
                let final_count = args.count.unwrap_or(1);

                Box::new(PasswordCommand::new(
                    final_length,
                    final_lower,
                    final_upper,
                    final_digits,
                    final_special,
                    final_extra,
                    final_repeat,
                    final_reuse,
                    final_exclude,
                    final_strategy,
                    final_count,
                    json,
                ))
            }
        }
    }
}
