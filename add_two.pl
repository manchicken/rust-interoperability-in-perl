#!/usr/bin/env perl

use v5.34;

use FFI::Platypus 2.00;
use FFI::CheckLib  qw( find_lib_or_die find_lib );
use File::Basename qw( dirname );

my $ffi = FFI::Platypus->new( api => 2, lang => 'Rust' );
$ffi->lib(find_lib_or_die(
  lib        => 'rust_part',
  libpath    => [dirname(__FILE__) . '/rust-part/target/debug'],
  systempath => [],
  recursive  => 1
)
);

$ffi->attach( add_two => ['u32', 'u32'] => 'u32' );

print add_two(12,23), "\n";  # prints 3
