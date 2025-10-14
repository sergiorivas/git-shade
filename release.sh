#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
BINARY_NAME=$(grep '^name = ' Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/')
REPO_URL=$(git config --get remote.origin.url | sed 's/\.git$//')

echo -e "${BLUE}🚀 Homebrew Formula Release Script${NC}\n"

# Step 1: Build binaries
echo -e "${GREEN}Step 1: Building binaries for macOS (Intel and ARM)${NC}"

gum spin --spinner dot --title "Installing Rust targets..." -- \
  rustup target add x86_64-apple-darwin aarch64-apple-darwin

echo "Building for x86_64 (Intel)..."
gum spin --spinner dot --title "Compiling for x86_64-apple-darwin..." -- \
  cargo build --release --target x86_64-apple-darwin

echo "Building for aarch64 (Apple Silicon)..."
gum spin --spinner dot --title "Compiling for aarch64-apple-darwin..." -- \
  cargo build --release --target aarch64-apple-darwin

echo -e "${GREEN}✓ Binaries built successfully${NC}\n"

# Step 2: Suggest next version
echo -e "${GREEN}Step 2: Determining next version${NC}"

LATEST_TAG=$(git tag --sort=-v:refname | head -1)

if [ -z "$LATEST_TAG" ]; then
  SUGGESTED_VERSION="1.0.0"
  echo "No existing tags found"
else
  echo "Latest tag: $LATEST_TAG"
  # Remove 'v' prefix if exists
  CLEAN_TAG=${LATEST_TAG#v}

  # Parse version parts
  IFS='.' read -r -a VERSION_PARTS <<< "$CLEAN_TAG"
  MAJOR=${VERSION_PARTS[0]}
  MINOR=${VERSION_PARTS[1]}
  PATCH=${VERSION_PARTS[2]}

  # Suggest patch increment
  SUGGESTED_VERSION="$MAJOR.$MINOR.$((PATCH + 1))"
fi

echo -e "\n${YELLOW}Suggested version: v$SUGGESTED_VERSION${NC}"
VERSION=$(gum input --placeholder "$SUGGESTED_VERSION" --value "$SUGGESTED_VERSION" --prompt "Version to release: ")

if [ -z "$VERSION" ]; then
  VERSION=$SUGGESTED_VERSION
fi

# Remove 'v' prefix if user added it
VERSION=${VERSION#v}
TAG="v$VERSION"

echo -e "${GREEN}✓ Will release version: $TAG${NC}\n"

# Step 3: Create archives
echo -e "${GREEN}Step 3: Creating release archives${NC}"

INTEL_ARCHIVE="${BINARY_NAME}-${TAG}-x86_64-apple-darwin.tar.gz"
ARM_ARCHIVE="${BINARY_NAME}-${TAG}-aarch64-apple-darwin.tar.gz"

echo "Creating archive for Intel..."
tar -czf "$INTEL_ARCHIVE" -C "target/x86_64-apple-darwin/release" "$BINARY_NAME"

echo "Creating archive for ARM..."
tar -czf "$ARM_ARCHIVE" -C "target/aarch64-apple-darwin/release" "$BINARY_NAME"

echo -e "${GREEN}✓ Archives created${NC}\n"

# Step 4: Calculate SHA256
echo -e "${GREEN}Step 4: Calculating SHA256 checksums${NC}"

INTEL_SHA256=$(shasum -a 256 "$INTEL_ARCHIVE" | awk '{print $1}')
ARM_SHA256=$(shasum -a 256 "$ARM_ARCHIVE" | awk '{print $1}')

echo "Intel SHA256: $INTEL_SHA256"
echo "ARM SHA256: $ARM_SHA256"
echo -e "${GREEN}✓ Checksums calculated${NC}\n"

# Step 5: Create git tag
echo -e "${GREEN}Step 5: Creating git tag${NC}"

if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo -e "${YELLOW}Warning: Tag $TAG already exists${NC}"
  OVERWRITE=$(gum confirm "Overwrite existing tag?" && echo "yes" || echo "no")

  if [ "$OVERWRITE" = "yes" ]; then
    git tag -d "$TAG"
    git push origin :refs/tags/"$TAG" 2>/dev/null || true
  else
    echo "Aborted"
    exit 1
  fi
fi

git tag -a "$TAG" -m "Release $TAG"
git push origin "$TAG"

echo -e "${GREEN}✓ Tag created and pushed${NC}\n"

# Step 6: Create GitHub release
echo -e "${GREEN}Step 6: Creating GitHub release${NC}"

RELEASE_NOTES=$(gum write --placeholder "Release notes (Ctrl+D to finish)..." --char-limit 0)

if [ -z "$RELEASE_NOTES" ]; then
  RELEASE_NOTES="Release $TAG"
fi

echo "Creating release on GitHub..."
gh release create "$TAG" \
  "$INTEL_ARCHIVE" \
  "$ARM_ARCHIVE" \
  --title "$TAG" \
  --notes "$RELEASE_NOTES"

echo -e "${GREEN}✓ GitHub release created${NC}\n"

# Step 7: Generate Homebrew formula
echo -e "${GREEN}Step 7: Generating Homebrew formula${NC}"

# Convert binary name to class name (e.g., my-app -> MyApp)
CLASS_NAME=$(echo "$BINARY_NAME" | sed -r 's/(^|-)(\w)/\U\2/g')

# Get description from Cargo.toml
DESCRIPTION=$(grep '^description = ' Cargo.toml | head -1 | sed 's/description = "\(.*\)"/\1/')

# Get license from Cargo.toml
LICENSE=$(grep '^license = ' Cargo.toml | head -1 | sed 's/license = "\(.*\)"/\1/')

FORMULA_FILE="${BINARY_NAME}.rb"

# Extract owner and repo from URL
if [[ $REPO_URL =~ github.com[:/](.+)/(.+)$ ]]; then
  GITHUB_OWNER="${BASH_REMATCH[1]}"
  GITHUB_REPO="${BASH_REMATCH[2]}"
else
  echo "Error: Could not parse GitHub repository URL"
  exit 1
fi

cat > "$FORMULA_FILE" << EOF
class $CLASS_NAME < Formula
  desc "$DESCRIPTION"
  homepage "$REPO_URL"
  version "$VERSION"
  license "$LICENSE"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/$GITHUB_OWNER/$GITHUB_REPO/releases/download/$TAG/$INTEL_ARCHIVE"
      sha256 "$INTEL_SHA256"
    else
      url "https://github.com/$GITHUB_OWNER/$GITHUB_REPO/releases/download/$TAG/$ARM_ARCHIVE"
      sha256 "$ARM_SHA256"
    end
  end

  def install
    bin.install "$BINARY_NAME"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/$BINARY_NAME --version")
  end
end
EOF

echo -e "${GREEN}✓ Formula generated: $FORMULA_FILE${NC}\n"

# Step 8: Commit and push formula
echo -e "${GREEN}Step 8: Committing and pushing formula${NC}"

git add "$FORMULA_FILE"

if git diff --cached --quiet; then
  echo "No changes to commit"
else
  git commit -m "Update Homebrew formula to $TAG"
  git push origin main || git push origin master
  echo -e "${GREEN}✓ Formula committed and pushed${NC}\n"
fi

# Step 9: Cleanup
echo -e "${GREEN}Step 9: Cleaning up temporary files${NC}"

rm -f "$INTEL_ARCHIVE" "$ARM_ARCHIVE"

echo -e "${GREEN}✓ Cleanup complete${NC}\n"

# Final summary
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Release process completed successfully!${NC}"
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo ""
echo "Version: $TAG"
echo "Formula: $FORMULA_FILE"
echo ""
echo "Users can install with:"
echo -e "${YELLOW}brew install https://raw.githubusercontent.com/$GITHUB_OWNER/$GITHUB_REPO/main/$FORMULA_FILE${NC}"
echo ""
echo -e "${BLUE}═══════════════════════════════════════${NC}"
