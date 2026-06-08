# Stuff to do for this API

## General

### API errors

Figure out how Poem handles errors - passing an invalid value to `operations`
panics, but Poem doesn't pick it up and turn it into a 400.

Try this https://deepwiki.com/poem-web/poem/2.5-error-handling

### DB or code?

How to handle curriculum values? Most sensible from a Rust perspective seems to
me to use the enums and structs to define what options exist, but most sensible
from a flexibility perspective is to use the database.

The issue is that the maths generator is algorithmic, and will entirely be
based on the code. The types of questions that show up will be based on the
stage and level and therefore what sort of operations the student is expected
to know at that stage, but that requires putting all known stages and levels
into the code.

Compare that to literally every other generator, which will pull questions from
the database, and therefore the values for subject, level, stage, and
difficulty can all be compared to the metadata for the questions and thus it's
just a filter on provided data. In that respect, the options for a curriculum
are flexible and can easily come from the database.

One option is to have each generator type able to provide its set of options,
which would imply that the API would require you to select the generator type
in order to know what options there are for the rest of it. This could be done
by requiring you to select your subject in order to know what curricula there
are available to it.

That means an enumeration of subjects is required, which will be a combination
of code and database options. We want the lib consumer to be able to decide
what subjects it actually interfaces with

#### Actual answer

The generator should be an object and it should hold strategy objects. We will
need a trait for strategy objects that can tell the generator what values it
will accept for generator params, including curricula (perhaps as a visitor
pattern, so the generator just asks it if it can handle a given params
object). One will be a fallback object, which will just get given the params,
so at least one object will not be allowed to answer no.

The user of the library will construct the generator and supply the strategy
objects, which means they can create their own as well.

The strategy objects will be able to report how you can structure their params,
so the user of the library can collate all the different options to present to
the end user, validate forms etc.

## Round 1

### API

- [ ] Question generator endpoint
    - [x] Maths
    - [ ] Pseudo-curricula without database (see round 2)

#### Generator interface

Questions can be gathered via `/questions` with a query string, e.g.

    GET /questions?subject=Maths

With the following parameters:

- subject - the subject of the curriculum
- count - how many to generate
- answer_count - how many answers to generate

#### Question object

The questions will be returned as an array of objects with the following
fields:

- text - the text of the question, i.e. the actual question being asked
- answers - an array of possible answers
- correct_answer - an integer identifying which answer is correct
- explanation - text explaining the answer, if appropriate

## Round 2

### API

- [ ] Other subjects
    - [ ] Ones that pull from database

### Database

#### Questions table

Just contains the question itself

##### Fields

- identifier
- text
- explanation text

#### Answers table

One-to-many with questions

##### Fields

- identifier
- text
- question ID
- correct?

#### Curricula table

Curricula contain a bunch of metadata to allow users to filter the question
down. A curriculum in this system is defined as any aspect of a question that
groups it with other questions with the same value for this aspect. It is
effectively a tagging system, but we want to keep the "tags" constrained to
common values that people might think of as a "curriculum" - actual real-world
curricula, but also pseudo-curricula like interest age.

Originally curricula were going to be a simple hierarchical structure, but it
quickly became apparent that they are actually orthogonal, since a single
question can be part of multiple curricula at the same time. We want clients
to be able to request either questions that are in all given curricula, or
from any given curriculum, thus either constraining or expanding the search
results.

This will be a many-to-many with questions and therefore require an extra
table.

##### Fields

- identifier
- name
- probably some helpful metadata for end users

#### Curriculum tags

This might be useful as a many-to-many against curricula. This would allow
us to attach arbitrary metadata to a curriculum, for example "this is an exam
board" or "this is the from national curriculum".

##### Fields

- tag

## Round 3

Use an ORM. I would have done this from the start but the options were a bit
overwhelming, and what I really want from an ORM is to create migrations from
my updates to the table definitions in the code, not to create manual
migrations that might not match what I actually did.
