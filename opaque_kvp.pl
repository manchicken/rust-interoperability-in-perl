#!/usr/bin/env perl

use v5.34;

use FFI::Platypus 2.00;
use FFI::CheckLib  qw( find_lib_or_die find_lib );
use File::Basename qw( dirname );

my $ffi = FFI::Platypus->new( api => 2, lang => 'Rust' );
$ffi->lib(
  find_lib_or_die(
    lib        => 'rust_part',
    libpath    => [ dirname(__FILE__) . '/rust-part/target/debug' ],
    systempath => [],
    recursive  => 1
  )
);

$ffi->type( 'object(OpaqueKeyValue)' => 'kvp_t' );
$ffi->attach( 'make_kvp'  => [ 'string', 'string' ] => 'kvp_t' );
$ffi->attach( 'print_kvp' => ['kvp_t'] );

# One with a value
{
  my $kvp = make_kvp( 'some key here', 'some value here' );
  say "PL: «${kvp}»";
  print_kvp($kvp);
};

# Empty string
{
  my $kvp = make_kvp( 'some key here', '' );
  say "PL: «${kvp}»";
  print_kvp($kvp);
};

# Undefined value but defined key
{
  my $kvp = make_kvp( 'some key here', undef );
  say "PL: «${kvp}»";
  print_kvp($kvp);
};

# Fully undefined
{
  my $kvp = make_kvp( undef, undef );
  say "PL: " . $kvp == undef ? 'KVP is undefined' : " . 'VP  == undef ? 'KVP is undefined' : 'KVP is something else : «${kvp}»'something else : «${kvp}»";
};
