class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.7"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.7/git-shade-v1.0.7-x86_64-apple-darwin.tar.gz"
      sha256 "5b6b7edc620b559039e4e927ab5faf5aac3dca35484dbd33cbffd28c7e4854b7"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.7/git-shade-v1.0.7-aarch64-apple-darwin.tar.gz"
      sha256 "fad4404fe21a24ef2534e6629538cf4003e40b08d67cb5cc68a7591bd9949376"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
