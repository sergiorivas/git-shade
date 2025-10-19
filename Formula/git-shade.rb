class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.5"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.5/git-shade-v1.0.5-x86_64-apple-darwin.tar.gz"
      sha256 "e3997241906e528b57a2df88c202846f9506f75230997faaa9949b351b012bc3"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.5/git-shade-v1.0.5-aarch64-apple-darwin.tar.gz"
      sha256 "ff59f274e1d4f588b7c5b3ff1b797f1f02ba493e8c601eeed02ab219a12ca3bc"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
