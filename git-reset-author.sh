#!/bin/sh

# Credits: http://stackoverflow.com/a/750191

git filter-branch -f --env-filter "
    GIT_AUTHOR_NAME='sir Random'
    GIT_AUTHOR_EMAIL='RandomAmbersky@gmail.com'
    GIT_COMMITTER_NAME='sir Random'
    GIT_COMMITTER_EMAIL='RandomAmbersky@gmail.com'
  " HEAD
