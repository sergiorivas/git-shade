class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.9"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.9/git-shade-v1.0.9-x86_64-apple-darwin.tar.gz"
      sha256 "92d47788332589147997d15dc150478042846ba304d9cf7f91b1752380a50da2"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.9/git-shade-v1.0.9-aarch64-apple-darwin.tar.gz"
      sha256 "9eaa4522d494e1a17dadcd253cd2c34487a2b4e89dfe181389f21be6b649b288"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
