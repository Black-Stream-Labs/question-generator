# Stuff to do for this API

## Round 1

### API

- [ ] Question generator endpoint
    - [ ] Maths
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
