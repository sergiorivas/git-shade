class GitShade < Formula
  desc ""
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.1"
  license ""

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.1/git-shade-v1.0.1-x86_64-apple-darwin.tar.gz"
      sha256 "c8e650909265b564de70b74f9500c3f30eddae99ed609bb7702b52b3b95af84f"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.1/git-shade-v1.0.1-aarch64-apple-darwin.tar.gz"
      sha256 "50304df394d1a0e1461efd28f8c6da2754438b1981ace03f17e279f58628950c"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
