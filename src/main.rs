#[macro_use]
extern crate clap;
extern crate resolv;

mod resolv_ext;

use resolv::{
	error::{
		Error,
		ResolutionError,
	},
	record::{
		A,
		AAAA,
	},
	Class,
	Resolver,
	Section,
};

use resolv_ext::{
	AddressRecord,
	ResponseExt,
};

use std::{
	ffi::OsStr,
	os::unix::ffi::OsStrExt,
};

#[repr(i32)]
enum ExitCode {
	Success = 0,       // Success or NODATA
	NameNotFound = 1,  // NXDOMAIN
	LookupFailure = 2, /* SRVFAIL, timeouts, failed parsing response, generic resolver failure... */
	RecordFailure = 3, // Failed parsing answer record
}

fn exit(code: ExitCode) -> ! {
	use std::process::exit as sys_exit;
	sys_exit(code as i32)
}

fn run<R: AddressRecord>(resolver: &mut Resolver, name: &OsStr) {
	match resolver.query(name.as_bytes(), Class::IN, R::get_record_type()) {
		Ok(mut response) => {
			for answer in response.records::<R>(Section::Answer) {
				match answer {
					Err(e) => {
						eprintln!(
							"Failed decoding address record for {:?}: {}",
							name, e
						);
						exit(ExitCode::RecordFailure);
					},
					Ok(answer) => {
						println!("{}", answer.data.address());
					},
				}
			}
		},
		Err(Error::Resolver(ResolutionError::HostNotFound)) => {
			eprintln!("Host not found: {:?}", name);
			// no need to check other address types for same name; this
			// error can be ignored if one doesn't care about it.
			exit(ExitCode::NameNotFound);
		},
		Err(Error::Resolver(ResolutionError::NoData)) => {
			// empty answer is perfectly fine
		},
		Err(e) => {
			eprintln!("Failed looking up {:?}: {}", name, e);
			exit(ExitCode::LookupFailure);
		},
	}
}

fn main() {
	let app = clap_app!(("lookup") =>
		(version: crate_version!())
		(author: crate_authors!("\n"))
		(about: crate_description!())
		(@arg IPv4: short("4") conflicts_with("IPv6") "Query only IPv4 records (A)")
		(@arg IPv6: short("6") "Query only IPv6 records (AAAA)")
		(@arg NAME: +required "Name to lookup")
	)
	.after_help(
		"Exit codes:

- 0: success (or NODATA).  You might want to treat an empty address
  set (no output) as failure too (similar to NXDOMAIN).
- 1: name not found (NXDOMAIN).  If an empty address set is ok for
  you, you might want to ignore this exit code.
- 2: SRVFAIL, timeouts, failed parsing response, generic resolver failure...
- 3: failed parsing a specific answer record (might have printed
  partial result, but breaks on first broken record)

Other exit codes should be treated as failures too; a non-zero exit code
always should show an error on stderr, and every time an error is
printed to stderr there should be a non-zero exit code.
",
	);
	let matches = app.get_matches();

	let ipv4_only = matches.is_present("IPv4");
	let ipv6_only = matches.is_present("IPv6");
	let name = matches.value_of_os("NAME").unwrap();

	let mut resolver = Resolver::new().unwrap_or_else(|| {
		eprintln!("Couldn't initialize resolver");
		exit(ExitCode::LookupFailure);
	});

	if !ipv6_only {
		run::<A>(&mut resolver, name);
	}

	if !ipv4_only {
		run::<AAAA>(&mut resolver, name);
	}

	exit(ExitCode::Success);
}
