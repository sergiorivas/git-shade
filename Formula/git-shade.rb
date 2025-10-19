class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.4"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.4/git-shade-v1.0.4-x86_64-apple-darwin.tar.gz"
      sha256 "e6254bd5168fa948fdbcf553a4c23d7b850244742564948d9bed9864b0f7eec4"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.4/git-shade-v1.0.4-aarch64-apple-darwin.tar.gz"
      sha256 "f607c9f9ab3b214048b3a09221cdfdd6fb1286968486798c843bbff42f5ea67e"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
