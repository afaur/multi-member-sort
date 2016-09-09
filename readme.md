## Multi Member Sort

[![Build Status](https://travis-ci.org/tinytacoteam/multi-member-sort.svg?branch=master)](https://travis-ci.org/tinytacoteam/multi-member-sort)

Picking the top of anything is hard. Humans are better at comparing two items at
a time, but it's even harder when you add multiple people.

Modify the list in `./data/` with each line being an option, then run the app:

~~~
cargo run
~~~

The app will prompt you asking how many people are ranking the list, then it'll
go through and rank all the items two at a time with each user, then average the
lists and show you the output.
