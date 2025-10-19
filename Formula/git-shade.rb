class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.6"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.6/git-shade-v1.0.6-x86_64-apple-darwin.tar.gz"
      sha256 "f86e3a55642f3bbd7225ba168b7bba8103b0b109207f0aefda64700d08f890cd"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.6/git-shade-v1.0.6-aarch64-apple-darwin.tar.gz"
      sha256 "058c77805013874d40ccb10d01ce6e213da7f00aad59646377e4c78d149dfbde"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
