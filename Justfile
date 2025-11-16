default: test

precommit: lint
precommit-ci: lint-ci
prepush: test

e:
    $EDITOR Justfile

install-hooks:
   pre-commit install

# -----------------------------------------------------------

demo-expand:
    #!/usr/bin/env echo-comment
    x=123
    # Value: $x
    echo "Value: $x"

# Justfile recipe using echo-comment (echo the comments)
demo-jf colour="blue":
    #!/usr/bin/env -S echo-comment --color {{colour}}
    set -euo pipefail
    # 🎉 Hello world from comment!
    python -c "print(f'{2+2=}')"
    # ✅ Mission complete

test:
    cargo nextest run

# Run the echo-comment example (comments become echoes)
[working-directory: 'examples']
demo-ec:
    bat example-echo-comment.sh
    ./example-echo-comment.sh

# Run the echo-comment-in-red example (comments become echoes, in red text)
[working-directory: 'examples']
demo-ec-red:
    bat example-echo-comment-in-red.sh
    ./example-echo-comment-in-red.sh

# Run the echo-comment-with-shell-flags example (comments become echoes, with shell flags)
[working-directory: 'examples']
demo-ec-shell-flags:
    bat example-echo-comment-with-shell-flags.sh
    ./example-echo-comment-with-shell-flags.sh

# Run the comment-echo example (echoes become comments) 
[working-directory: 'examples']
demo-ce:
    bat example-echo-comment.sh
    ./example-comment-echo.sh

# ------------------------------------------------------------

clip:
    cargo clippy --tests

ship:
    #!/usr/bin/env -S echo-comment --shell-flags="-euo pipefail" --color red
    ## Refuse to run if not on master branch or not up to date with origin/master
    branch="$(git rev-parse --abbrev-ref HEAD)"
    if [[ "$branch" != "master" ]]; then
        # ❌ Refusing to run: not on 'master' branch (current: $branch)
    exit 1
    fi
    git fetch origin master
    local_rev="$(git rev-parse HEAD)"
    remote_rev="$(git rev-parse origin/master)"
    if [[ "$local_rev" != "$remote_rev" ]]; then
        # ❌ Refusing to run: local master branch is not up to date with origin/master
        # Local HEAD:  $local_rev
        # Origin HEAD: $remote_rev
        # Please pull/rebase to update.
    exit 1
    fi
    release-plz update
    git add .
    git commit -m "chore(release): Upgrades"
    git push
    just publish

publish:
    #!/usr/bin/env -S bash -euo pipefail
    git_token=$(gh auth token 2>/dev/null) || git_token=$PUBLISH_GITHUB_TOKEN
    release-plz release --backend github --git-token $git_token

# ------------------------------------------------------------

fix-eof-ws mode="":
    #!/usr/bin/env sh
    ARGS=''
    if [ "{{mode}}" = "check" ]; then
        ARGS="--check-only"
    fi
    whitespace-format --add-new-line-marker-at-end-of-file \
          --new-line-marker=linux \
          --normalize-new-line-markers \
          --exclude ".git/|target/|dist/|.json$|.lock$|.parquet$|.venv/|.stubs/|\..*cache/" \
          $ARGS \
          .

lint:
    #!/usr/bin/env echo-comment
    # 🧹 Begin linting!
    just lint-action
    just lint-ci

lint-action:
    actionlint .github/workflows/CI.yml

lint-ci:
    taplo lint
    taplo format --check
    just fix-eof-ws check
    cargo machete
    cargo fmt --check --all

fmt:
    taplo lint
    taplo format
    just fix-eof-ws
    cargo machete
    cargo fmt --all
