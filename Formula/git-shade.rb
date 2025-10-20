class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.10"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.10/git-shade-v1.0.10-x86_64-apple-darwin.tar.gz"
      sha256 "cffc7276b7ae41eeb64fafd7f1c0ed810cd520de89596f791ce0086f047d74da"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.10/git-shade-v1.0.10-aarch64-apple-darwin.tar.gz"
      sha256 "83b7c6c5c949ea56a37ce7bb5a302e690c298de25e2fd70cb62d87ec26b1cf23"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
