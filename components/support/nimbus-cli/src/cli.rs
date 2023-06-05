// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    author,
    long_about = r#"Mozilla Nimbus' command line tool for mobile apps"#
)]
pub(crate) struct Cli {
    /// The app name according to Nimbus.
    #[arg(short, long, value_name = "APP")]
    pub(crate) app: String,

    /// The channel according to Nimbus. This determines which app to talk to.
    #[arg(short, long, value_name = "CHANNEL")]
    pub(crate) channel: String,

    /// The device id of the simulator, emulator or device.
    #[arg(short, long, value_name = "DEVICE_ID")]
    pub(crate) device_id: Option<String>,

    #[command(subcommand)]
    pub(crate) command: CliCommand,
}

#[derive(Subcommand, Clone)]
pub(crate) enum CliCommand {
    /// Send a complete JSON file to the Nimbus SDK and apply it immediately.
    ApplyFile {
        /// The filename to be loaded into the SDK.
        file: PathBuf,

        /// Keeps existing enrollments and experiments before enrolling.
        ///
        /// This is unlikely what you want to do.
        #[arg(long, default_value = "false")]
        preserve_nimbus_db: bool,
    },

    /// Capture the logs into a file.
    CaptureLogs {
        /// The file to put the logs.
        file: PathBuf,
    },

    /// Enroll into an experiment or a rollout.
    ///
    /// The experiment slug is a combination of the actual slug, and the server it came from.
    ///
    /// * `release`/`stage` determines the server.
    ///
    /// * `preview` selects the preview collection.
    ///
    /// These can be further combined: e.g. $slug, preview/$slug, stage/$slug, stage/preview/$slug
    Enroll {
        /// The experiment slug, including the server and collection.
        #[arg(value_name = "SLUG")]
        experiment: String,

        /// The branch slug.
        #[arg(short, long, value_name = "BRANCH")]
        branch: String,

        /// Optional rollout slugs, including the server and collection.
        #[arg(value_name = "ROLLOUTS")]
        rollouts: Vec<String>,

        /// Preserves the original experiment targeting
        #[arg(long, default_value = "false")]
        preserve_targeting: bool,

        /// Preserves the original experiment bucketing
        #[arg(long, default_value = "false")]
        preserve_bucketing: bool,

        #[command(flatten)]
        open: OpenArgs,

        /// Keeps existing enrollments and experiments before enrolling.
        ///
        /// This is unlikely what you want to do.
        #[arg(long, default_value = "false")]
        preserve_nimbus_db: bool,

        /// Instead of fetching from the server, use a file instead
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Don't validate the feature config files before enrolling
        #[arg(long, default_value = "false")]
        no_validate: bool,

        #[command(flatten)]
        manifest: ManifestArgs,
    },

    /// Fetch one or more experiments and put it in a file.
    Fetch {
        /// The file to download the recipes to.
        file: PathBuf,

        /// An optional server slug, e.g. release or stage/preview.
        #[arg(long, short, value_name = "SERVER", default_value = "")]
        server: String,

        /// The recipe slugs, including server.
        ///
        /// Use once per recipe to download. e.g.
        /// fetch file.json -r preview/my-experiment -r my-rollout
        ///
        /// Cannot be used with the server option.
        #[arg(long = "recipe", short, value_name = "RECIPE")]
        recipes: Vec<String>,
    },

    /// List the experiments from a server
    List {
        /// A server slug e.g. preview, release, stage, stage/preview
        server: Option<String>,

        /// An optional file
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    },

    /// Print the state of the Nimbus database to logs.
    ///
    /// This causes a restart of the app.
    LogState,

    /// Open the app without changing the state of experiment enrollments.
    Open {
        #[command(flatten)]
        open: OpenArgs,

        /// By default, the app is terminated before sending the a deeplink.
        ///
        /// If this flag is set, then do not terminate the app if it is already runnning.
        #[arg(long, default_value = "false")]
        no_clobber: bool,
    },

    /// Reset the app back to its just installed state
    ResetApp,

    /// Follow the logs for the given app.
    TailLogs,

    /// Configure an application feature with one or more feature config files.
    ///
    /// One file per branch. The branch slugs will correspond to the file names.
    ///
    /// By default, the files are validated against the manifest; this can be
    /// overridden with `--no-validate`.
    TestFeature {
        /// The identifier of the feature to configure
        feature_id: String,

        /// One or more files containing a feature config for the feature.
        files: Vec<PathBuf>,

        #[command(flatten)]
        open: OpenArgs,

        /// Don't validate the feature config files before enrolling
        #[arg(long, default_value = "false")]
        no_validate: bool,

        #[command(flatten)]
        manifest: ManifestArgs,
    },

    /// Unenroll from all experiments and rollouts
    Unenroll,

    /// Validate an experiment against a feature manifest
    Validate {
        /// The experiment slug, including the server and collection.
        #[arg(value_name = "SLUG")]
        experiment: String,

        /// An optional file from which to get the experiment
        #[arg(long, value_name = "EXPERIMENTS_FILE")]
        file: Option<PathBuf>,

        #[command(flatten)]
        manifest: ManifestArgs,
    },
}

#[derive(Args, Clone, Debug, Default)]
pub(crate) struct ManifestArgs {
    /// An optional manifest file
    #[arg(long, value_name = "MANIFEST_FILE")]
    pub(crate) manifest: Option<String>,

    /// An optional version of the app.
    /// If present, constructs the `ref` from an app specific template.
    /// Due to inconsistencies in branching names, this isn't always
    /// reliable.
    #[arg(long, value_name = "APP_VERSION")]
    pub(crate) version: Option<String>,

    /// The branch/tag/commit for the version of the manifest
    /// to get from Github.
    #[arg(long, value_name = "APP_VERSION", default_value = "main")]
    pub(crate) ref_: String,
}

#[derive(Args, Clone, Debug, Default)]
pub(crate) struct OpenArgs {
    /// Optional deeplink. If present, launch with this link.
    #[arg(long, value_name = "DEEPLINK")]
    pub(crate) deeplink: Option<String>,

    /// Resets the app back to its initial state before launching
    #[arg(long, default_value = "false")]
    pub(crate) reset_app: bool,
}
