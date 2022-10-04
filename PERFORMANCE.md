| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 31.2 ± 0.4 | 30.7 | 41.8 | 16.65 ± 1.51 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.9 ± 0.2 | 1.6 | 2.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 31.0 ± 1.0 | 30.6 | 46.7 | 16.83 ± 1.63 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.8 ± 0.2 | 1.6 | 3.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 31.1 ± 0.2 | 30.6 | 32.8 | 16.95 ± 1.38 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.8 ± 0.1 | 1.6 | 2.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 151.6 ± 1.0 | 150.5 | 161.4 | 1.32 ± 0.02 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 114.8 ± 1.7 | 111.5 | 142.5 | 1.00 |

