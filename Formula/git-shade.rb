class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.11"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.11/git-shade-v1.0.11-x86_64-apple-darwin.tar.gz"
      sha256 "10bbde2fc1e255d0ff118c31de65f91b94bdcf9e7ad64fe2f2a9cc8dee222365"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.11/git-shade-v1.0.11-aarch64-apple-darwin.tar.gz"
      sha256 "76a7e0e0efe1546ba4629398bb9a1193efaaedc78eed4b692e0c2aa8df2c7c16"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
