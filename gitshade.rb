class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.3"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.3/git-shade-v1.0.3-x86_64-apple-darwin.tar.gz"
      sha256 "3873d35179992eaf29a994212c215f0c6abbfb0b10ab92482665201be135547a"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.3/git-shade-v1.0.3-aarch64-apple-darwin.tar.gz"
      sha256 "e129b9df3791da17b99bbf33aee25d0712709168bee1a9325762b74cbf3623f5"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
