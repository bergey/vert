{ lib, buildRustCrate, buildRustCrateHelpers }:
with buildRustCrateHelpers;
let inherit (lib.lists) fold;
    inherit (lib.attrsets) recursiveUpdate;
in
rec {

# kernel32-sys-0.2.2

  crates.kernel32_sys."0.2.2" = deps: { features?(features_.kernel32_sys."0.2.2" deps {}) }: buildRustCrate {
    crateName = "kernel32-sys";
    version = "0.2.2";
    description = "Contains function definitions for the Windows API library kernel32. See winapi for types and constants.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "1lrw1hbinyvr6cp28g60z97w32w8vsk6pahk64pmrv2fmby8srfj";
    libName = "kernel32";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."winapi"."${deps."kernel32_sys"."0.2.2"."winapi"}" deps)
    ]);

    buildDependencies = mapFeatures features ([
      (crates."winapi_build"."${deps."kernel32_sys"."0.2.2"."winapi_build"}" deps)
    ]);
  };
  features_.kernel32_sys."0.2.2" = deps: f: updateFeatures f (rec {
    kernel32_sys."0.2.2".default = (f.kernel32_sys."0.2.2".default or true);
    winapi."${deps.kernel32_sys."0.2.2".winapi}".default = true;
    winapi_build."${deps.kernel32_sys."0.2.2".winapi_build}".default = true;
  }) [
    (features_.winapi."${deps."kernel32_sys"."0.2.2"."winapi"}" deps)
    (features_.winapi_build."${deps."kernel32_sys"."0.2.2"."winapi_build"}" deps)
  ];


# end
# libc-0.2.62

  crates.libc."0.2.62" = deps: { features?(features_.libc."0.2.62" deps {}) }: buildRustCrate {
    crateName = "libc";
    version = "0.2.62";
    description = "Raw FFI bindings to platform libraries like libc.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1vsb4pyn6gl6sri6cv5hin5wjfgk7lk2bshzmxb1xnkckjhz4gbx";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."libc"."0.2.62" or {});
  };
  features_.libc."0.2.62" = deps: f: updateFeatures f (rec {
    libc = fold recursiveUpdate {} [
      { "0.2.62"."align" =
        (f.libc."0.2.62"."align" or false) ||
        (f.libc."0.2.62".rustc-dep-of-std or false) ||
        (libc."0.2.62"."rustc-dep-of-std" or false); }
      { "0.2.62"."rustc-std-workspace-core" =
        (f.libc."0.2.62"."rustc-std-workspace-core" or false) ||
        (f.libc."0.2.62".rustc-dep-of-std or false) ||
        (libc."0.2.62"."rustc-dep-of-std" or false); }
      { "0.2.62"."std" =
        (f.libc."0.2.62"."std" or false) ||
        (f.libc."0.2.62".default or false) ||
        (libc."0.2.62"."default" or false) ||
        (f.libc."0.2.62".use_std or false) ||
        (libc."0.2.62"."use_std" or false); }
      { "0.2.62".default = (f.libc."0.2.62".default or true); }
    ];
  }) [];


# end
# term_size-0.3.1

  crates.term_size."0.3.1" = deps: { features?(features_.term_size."0.3.1" deps {}) }: buildRustCrate {
    crateName = "term_size";
    version = "0.3.1";
    description = "functions for determining terminal sizes and dimensions";
    authors = [ "Kevin K. <kbknapp@gmail.com>" "Benjamin Sago <ogham@bsago.me>" ];
    sha256 = "08gjw2a7igprgw7jjkf8011h320snjqabbn7nhycq924pqyawqw3";
    dependencies = mapFeatures features ([
])
      ++ (if !(kernel == "windows") then mapFeatures features ([
      (crates."libc"."${deps."term_size"."0.3.1"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."kernel32_sys"."${deps."term_size"."0.3.1"."kernel32_sys"}" deps)
      (crates."winapi"."${deps."term_size"."0.3.1"."winapi"}" deps)
    ]) else []);
    features = mkFeatures (features."term_size"."0.3.1" or {});
  };
  features_.term_size."0.3.1" = deps: f: updateFeatures f (rec {
    kernel32_sys."${deps.term_size."0.3.1".kernel32_sys}".default = true;
    libc."${deps.term_size."0.3.1".libc}".default = true;
    term_size = fold recursiveUpdate {} [
      { "0.3.1"."clippy" =
        (f.term_size."0.3.1"."clippy" or false) ||
        (f.term_size."0.3.1".lints or false) ||
        (term_size."0.3.1"."lints" or false); }
      { "0.3.1"."lints" =
        (f.term_size."0.3.1"."lints" or false) ||
        (f.term_size."0.3.1".travis or false) ||
        (term_size."0.3.1"."travis" or false); }
      { "0.3.1"."nightly" =
        (f.term_size."0.3.1"."nightly" or false) ||
        (f.term_size."0.3.1".lints or false) ||
        (term_size."0.3.1"."lints" or false) ||
        (f.term_size."0.3.1".travis or false) ||
        (term_size."0.3.1"."travis" or false); }
      { "0.3.1".default = (f.term_size."0.3.1".default or true); }
    ];
    winapi."${deps.term_size."0.3.1".winapi}".default = true;
  }) [
    (features_.libc."${deps."term_size"."0.3.1"."libc"}" deps)
    (features_.kernel32_sys."${deps."term_size"."0.3.1"."kernel32_sys"}" deps)
    (features_.winapi."${deps."term_size"."0.3.1"."winapi"}" deps)
  ];


# end
# termios-0.3.1

  crates.termios."0.3.1" = deps: { features?(features_.termios."0.3.1" deps {}) }: buildRustCrate {
    crateName = "termios";
    version = "0.3.1";
    description = "Safe bindings for the termios library.";
    authors = [ "David Cuddeback <david.cuddeback@gmail.com>" ];
    sha256 = "1h0fwglrhay85fkbl05ym5gh8hxzl7pyz0a51zfmmngxrf7823c2";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."termios"."0.3.1"."libc"}" deps)
    ]);
  };
  features_.termios."0.3.1" = deps: f: updateFeatures f (rec {
    libc."${deps.termios."0.3.1".libc}".default = true;
    termios."0.3.1".default = (f.termios."0.3.1".default or true);
  }) [
    (features_.libc."${deps."termios"."0.3.1"."libc"}" deps)
  ];


# end
# winapi-0.2.8

  crates.winapi."0.2.8" = deps: { features?(features_.winapi."0.2.8" deps {}) }: buildRustCrate {
    crateName = "winapi";
    version = "0.2.8";
    description = "Types and constants for WinAPI bindings. See README for list of crates providing function bindings.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "0a45b58ywf12vb7gvj6h3j264nydynmzyqz8d8rqxsj6icqv82as";
  };
  features_.winapi."0.2.8" = deps: f: updateFeatures f (rec {
    winapi."0.2.8".default = (f.winapi."0.2.8".default or true);
  }) [];


# end
# winapi-build-0.1.1

  crates.winapi_build."0.1.1" = deps: { features?(features_.winapi_build."0.1.1" deps {}) }: buildRustCrate {
    crateName = "winapi-build";
    version = "0.1.1";
    description = "Common code for build.rs in WinAPI -sys crates.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "1lxlpi87rkhxcwp2ykf1ldw3p108hwm24nywf3jfrvmff4rjhqga";
    libName = "build";
  };
  features_.winapi_build."0.1.1" = deps: f: updateFeatures f (rec {
    winapi_build."0.1.1".default = (f.winapi_build."0.1.1".default or true);
  }) [];


# end
}
