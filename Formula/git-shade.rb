class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.8"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.8/git-shade-v1.0.8-x86_64-apple-darwin.tar.gz"
      sha256 "449c3a495fc5fc21a33bf243c08af85b73b1b2fb0c18c4d7de970a49c9f7885b"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.8/git-shade-v1.0.8-aarch64-apple-darwin.tar.gz"
      sha256 "37ed9c58618b9b383c9873b15d7da73374150b46e6c1073ad945146ce322514e"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
