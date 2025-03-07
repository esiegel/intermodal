use crate::common::*;

const INPUT_HELP: &str = "Show information about torrent at `INPUT`. If `INPUT` is `-`, read \
                          torrent metainfo from standard input.";

const INPUT_FLAG: &str = "input-flag";

const INPUT_POSITIONAL: &str = "<INPUT>";

const INPUT_VALUE: &str = "INPUT";

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Display information about a .torrent file.")
)]
pub(crate) struct Show {
  #[structopt(
    name = INPUT_FLAG,
    long = "input",
    short = "i",
    value_name = INPUT_VALUE,
    empty_values(false),
    parse(try_from_os_str = InputTarget::try_from_os_str),
    help = INPUT_HELP,
  )]
  input_flag: Option<InputTarget>,
  #[structopt(
    name = INPUT_POSITIONAL,
    value_name = INPUT_VALUE,
    empty_values(false),
    parse(try_from_os_str = InputTarget::try_from_os_str),
    required_unless = INPUT_FLAG,
    conflicts_with = INPUT_FLAG,
    help = INPUT_HELP,
  )]
  input_positional: Option<InputTarget>,
}

impl Show {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let target = xor_args(
      "input_flag",
      &self.input_flag,
      "input_positional",
      &self.input_positional,
    )?;

    let input = env.read(target)?;
    let summary = TorrentSummary::from_input(&input)?;
    summary.write(env)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn input_required() {
    test_env! {
      args: [
        "torrent",
        "show",
      ],
      tree: {
      },
      matches: Err(Error::Clap { .. }),
    };

    test_env! {
      args: [
        "torrent",
        "show",
        "--input",
        "foo",
      ],
      tree: {
      },
      matches: Err(Error::Filesystem { .. }),
    };

    test_env! {
      args: [
        "torrent",
        "show",
        "foo",
      ],
      tree: {
      },
      matches: Err(Error::Filesystem { .. }),
    };

    test_env! {
      args: [
        "torrent",
        "show",
        "--input",
        "foo",
        "foo",
      ],
      tree: {
      },
      matches: Err(Error::Clap { .. }),
    };
  }

  #[test]
  fn output() -> Result<()> {
    let metainfo = Metainfo {
      announce: Some("announce".into()),
      announce_list: Some(vec![vec!["announce".into(), "b".into()], vec!["c".into()]]),
      nodes: Some(vec![
        "x:12".parse().unwrap(),
        "1.1.1.1:16".parse().unwrap(),
        "[2001:0db8:85a3::0000:8a2e:0370]:7334".parse().unwrap(),
      ]),
      comment: Some("comment".into()),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: PieceList::from_pieces(&["xyz", "abc"]),
        mode: Mode::Single {
          length: Bytes(20),
          md5sum: None,
        },
      },
    };

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      let want = "         Name  foo
      Comment  comment
Creation Date  1970-01-01 00:00:01 UTC
   Created By  created by
       Source  source
    Info Hash  e12253978dc6d50db11d05747abcea1ad03b51c5
 Torrent Size  339 bytes
 Content Size  20 bytes
      Private  yes
      Tracker  announce
Announce List  Tier 1: announce
                       b
               Tier 2: c
    DHT Nodes  x:12
               1.1.1.1:16
               [2001:db8:85a3::8a2e:370]:7334
   Piece Size  16 KiB
  Piece Count  2
   File Count  1
        Files  foo
";

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      let want = "         Name  foo
      Comment  comment
Creation Date  1970-01-01 00:00:01 UTC
   Created By  created by
       Source  source
    Info Hash  e12253978dc6d50db11d05747abcea1ad03b51c5
 Torrent Size  339 bytes
 Content Size  20 bytes
      Private  yes
      Tracker  announce
Announce List  Tier 1: announce
                       b
               Tier 2: c
    DHT Nodes  x:12
               1.1.1.1:16
               [2001:db8:85a3::8a2e:370]:7334
   Piece Size  16 KiB
  Piece Count  2
   File Count  1
        Files  foo
";

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      let want = "\
name\tfoo
comment\tcomment
creation date\t1970-01-01 00:00:01 UTC
created by\tcreated by
source\tsource
info hash\te12253978dc6d50db11d05747abcea1ad03b51c5
torrent size\t339
content size\t20
private\tyes
tracker\tannounce
announce list\tannounce\tb\tc
dht nodes\tx:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
piece size\t16384
piece count\t2
file count\t1
files\tfoo
";

      assert_eq!(have, want);
    }

    Ok(())
  }

  #[test]
  fn tier_list_with_main() -> Result<()> {
    let metainfo = Metainfo {
      announce: Some("a".into()),
      announce_list: Some(vec![vec!["x".into()], vec!["y".into()], vec!["z".into()]]),
      comment: Some("comment".into()),
      created_by: Some("created by".into()),
      nodes: Some(vec![
        "x:12".parse().unwrap(),
        "1.1.1.1:16".parse().unwrap(),
        "[2001:0db8:85a3::0000:8a2e:0370]:7334".parse().unwrap(),
      ]),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: PieceList::from_pieces(&["xyz", "abc"]),
        mode: Mode::Single {
          length: Bytes(20),
          md5sum: None,
        },
      },
    };

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      let want = "         Name  foo
      Comment  comment
Creation Date  1970-01-01 00:00:01 UTC
   Created By  created by
       Source  source
    Info Hash  e12253978dc6d50db11d05747abcea1ad03b51c5
 Torrent Size  327 bytes
 Content Size  20 bytes
      Private  yes
      Tracker  a
Announce List  Tier 1: x
               Tier 2: y
               Tier 3: z
    DHT Nodes  x:12
               1.1.1.1:16
               [2001:db8:85a3::8a2e:370]:7334
   Piece Size  16 KiB
  Piece Count  2
   File Count  1
        Files  foo
";

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      let want = "\
name\tfoo
comment\tcomment
creation date\t1970-01-01 00:00:01 UTC
created by\tcreated by
source\tsource
info hash\te12253978dc6d50db11d05747abcea1ad03b51c5
torrent size\t327
content size\t20
private\tyes
tracker\ta
announce list\tx\ty\tz
dht nodes\tx:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
piece size\t16384
piece count\t2
file count\t1
files\tfoo
";

      assert_eq!(have, want);
    }

    Ok(())
  }

  #[test]
  fn tier_list_without_main() -> Result<()> {
    let metainfo = Metainfo {
      announce: Some("a".into()),
      announce_list: Some(vec![vec!["b".into()], vec!["c".into()], vec!["a".into()]]),
      comment: Some("comment".into()),
      nodes: Some(vec![
        "x:12".parse().unwrap(),
        "1.1.1.1:16".parse().unwrap(),
        "[2001:0db8:85a3::8a2e:0370]:7334".parse().unwrap(),
      ]),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: PieceList::from_pieces(&["abc"]),
        mode: Mode::Single {
          length: Bytes(20),
          md5sum: None,
        },
      },
    };

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      let want = "         Name  foo
      Comment  comment
Creation Date  1970-01-01 00:00:01 UTC
   Created By  created by
       Source  source
    Info Hash  b9cd9cae5748518c99d00d8ae86c0162510be4d9
 Torrent Size  307 bytes
 Content Size  20 bytes
      Private  yes
      Tracker  a
Announce List  Tier 1: b
               Tier 2: c
               Tier 3: a
    DHT Nodes  x:12
               1.1.1.1:16
               [2001:db8:85a3::8a2e:370]:7334
   Piece Size  16 KiB
  Piece Count  1
   File Count  1
        Files  foo
";

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      let want = "\
name\tfoo
comment\tcomment
creation date\t1970-01-01 00:00:01 UTC
created by\tcreated by
source\tsource
info hash\tb9cd9cae5748518c99d00d8ae86c0162510be4d9
torrent size\t307
content size\t20
private\tyes
tracker\ta
announce list\tb\tc\ta
dht nodes\tx:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
piece size\t16384
piece count\t1
file count\t1
files\tfoo
";

      assert_eq!(have, want);
    }

    Ok(())
  }

  #[test]
  fn trackerless() -> Result<()> {
    let metainfo = Metainfo {
      announce: None,
      announce_list: None,
      comment: Some("comment".into()),
      nodes: Some(vec![
        "x:12".parse().unwrap(),
        "1.1.1.1:16".parse().unwrap(),
        "[2001:0db8:85a3::8a2e:0370]:7334".parse().unwrap(),
      ]),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: PieceList::from_pieces(&["abc"]),
        mode: Mode::Single {
          length: Bytes(20),
          md5sum: None,
        },
      },
    };

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      let want = "         Name  foo
      Comment  comment
Creation Date  1970-01-01 00:00:01 UTC
   Created By  created by
       Source  source
    Info Hash  b9cd9cae5748518c99d00d8ae86c0162510be4d9
 Torrent Size  261 bytes
 Content Size  20 bytes
      Private  yes
    DHT Nodes  x:12
               1.1.1.1:16
               [2001:db8:85a3::8a2e:370]:7334
   Piece Size  16 KiB
  Piece Count  1
   File Count  1
        Files  foo
";

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      let want = "\
name\tfoo
comment\tcomment
creation date\t1970-01-01 00:00:01 UTC
created by\tcreated by
source\tsource
info hash\tb9cd9cae5748518c99d00d8ae86c0162510be4d9
torrent size\t261
content size\t20
private\tyes
dht nodes\tx:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
piece size\t16384
piece count\t1
file count\t1
files\tfoo
";

      assert_eq!(have, want);
    }

    Ok(())
  }
}
