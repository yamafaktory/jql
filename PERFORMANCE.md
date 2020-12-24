| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 3.4 ± 0.5 | 2.9 | 8.6 | 1.70 ± 0.63 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.0 ± 0.7 | 1.5 | 10.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 3.1 ± 0.7 | 2.6 | 14.3 | 1.57 ± 0.56 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.0 ± 0.5 | 1.5 | 10.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 3.1 ± 0.6 | 2.6 | 9.8 | 1.52 ± 0.70 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.0 ± 0.9 | 1.5 | 15.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '.[] \| .name, .url, .language, .stargazers_count, .watchers_count'` | 158.1 ± 5.4 | 147.6 | 213.4 | 1.00 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}'` | 4817.4 ± 134.3 | 4181.8 | 5461.4 | 30.46 ± 1.34 |

