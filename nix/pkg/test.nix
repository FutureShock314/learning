let
  addOne = n: n + 1;
  vars = {};
in {
  vars.a = ( addOne 5 );
  vars.b = ( addOne vars.a );
  vars.c = ( addOne vars.b );
}
