#!/usr/bin/env bash
# content/post を srn221B/blog の post/ に反映する PR を作る。
#
#   ./scripts/sync-blog.sh            # 差分があれば PR を作る
#   DRY_RUN=1 ./scripts/sync-blog.sh  # 差分を表示するだけ（push も PR もしない）
#
# gh にログイン済みであること。このリポジトリを正として、blog 側のファイルを揃える
# （content/post で消した記事は blog からも消える）。
set -euo pipefail
cd "$(dirname "$0")/.."

REPO="srn221B/blog"
BASE="master"
GIT=(git -c "credential.helper=!gh auth git-credential")

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

"${GIT[@]}" clone -q --depth 1 --branch "$BASE" "https://github.com/$REPO.git" "$work"
rsync -a --delete content/post/ "$work/post/"

cd "$work"
if [[ -z "$(git status --porcelain post)" ]]; then
  echo "差分はありません。"
  exit 0
fi

git status --short post
if [[ "${DRY_RUN:-}" == "1" ]]; then
  echo
  echo "DRY_RUN=1 のため、ここで終了します。"
  exit 0
fi

branch="sync-posts-$(date +%Y%m%d-%H%M%S)"
git checkout -q -b "$branch"
git add post
git commit -q -m "Sync posts from srn221B.github.io"
"${GIT[@]}" push -q -u origin "$branch"

changes="$(git show --name-status --format= HEAD | sed 's/^/- `/; s/\t/` /')"
gh pr create --repo "$REPO" --base "$BASE" --head "$branch" \
  --title "Sync posts from srn221B.github.io" --body-file - <<BODY
## Summary
srn221B.github.io の \`content/post\` に合わせて \`post/\` を更新します。

$changes
BODY
