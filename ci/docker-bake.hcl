target "clippy" {
  context    = "."
  dockerfile = "./ci/Dockerfile.clippy"
  cache-from = [
    {
      type = "gha",
      scope = "clippy-cache",
    },
    {
      type = "inline",
    }
  ]
  cache-to = [
    {
      type = "gha",
      mode = "max",
      scope = "clippy-cache",
    },
    {
      type = "inline",
    }
  ]
  tags = [
    "clippy:latest",
  ]
}

target "test" {
  context    = "."
  dockerfile = "./ci/Dockerfile.test"
  cache-from = [
    {
      type = "gha",
      scope = "test-cache",
    },
    {
      type = "inline",
    }
  ]
  cache-to = [
    {
      type = "gha",
      mode = "max",
      scope = "test-cache",
    },
    {
      type = "inline",
    }
  ]
  tags = [
    "test:latest",
  ]
}