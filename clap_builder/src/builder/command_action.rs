/// Behavior of commands when they are encountered while parsing
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "help")] {
/// # use clap_builder as clap;
/// # use clap::Command;
/// # use clap::Arg;
/// let cmd = Command::new("mycmd")
///     .subcommand(
///         Command::new("aire")
///             .command_action(clap::CommandAction::Help)
///     );
///
/// // Existing help still exists
/// let err = cmd.clone().try_get_matches_from(["mycmd", "help"]).unwrap_err();
/// assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
///
/// // New help available
/// let err = cmd.try_get_matches_from(["mycmd", "aire"]).unwrap_err();
/// assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
/// # }
/// ```
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
#[allow(missing_copy_implementations)] // In the future, we may accept `Box<dyn ...>`
pub enum CommandAction {
    /// When encountered, expect the user to handle the command action.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap_builder as clap;
    /// # use clap::Command;
    /// # use clap::Arg;
    /// let cmd = Command::new("mycmd")
    ///     .subcommand(
    ///         Command::new("cmd")
    ///             .command_action(clap::CommandAction::User)
    ///     );
    ///
    /// let matches = cmd.try_get_matches_from(vec!["mycmd", "cmd"]).unwrap();
    /// assert!(matches.subcommand().is_some());
    /// ```
    #[default]
    User,
    /// When encountered, display [`Command::print_help`][super::Command::print_help]
    ///
    /// Depending on the flag, [`Command::print_long_help`][super::Command::print_long_help] may be shown
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "help")] {
    /// # use clap_builder as clap;
    /// # use clap::Command;
    /// let cmd = Command::new("mycmd")
    ///     .subcommand(
    ///         Command::new("aire")
    ///             .command_action(clap::CommandAction::Help)
    ///     );
    ///
    /// // Existing help still exists
    /// let err = cmd.clone().try_get_matches_from(["mycmd", "help"]).unwrap_err();
    /// assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    ///
    /// // New help available
    /// let err = cmd.try_get_matches_from(["mycmd", "aire"]).unwrap_err();
    /// assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    /// # }
    /// ```
    Help,
}
